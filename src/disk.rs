use crate::chs::CHS;
use crate::fs::vbr::VBR;
use crate::partition::Partition;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::*;
use std::path::PathBuf;

/// A Disk is the holding structure for a collection of Sectors. It also
/// represents the interface between what the emulator gets to see, and what is
/// present as a file on the host computer.
#[derive(Debug)]
pub struct Disk {
    pub(crate) bootcode: [u8; 446],
    pub(crate) geometry: CHS,
    pub(crate) partitions: Vec<Partition>,
    pub(crate) path: PathBuf,
    pub(crate) size: u64,
    pub(crate) sector_count: u64,
    sectors: Vec<Sector>,
}

/// Data structure for individual sectors. A sector holds 512 bytes of data and is
/// the smallest unit of data a Disk can work with. The data is kept in a Vec<u8> internally.
/// The position of the sector is the LBA address and we keep a 'dirty' flag to see if the
/// sector is present on the disk.
#[derive(Debug)]
pub struct Sector {
    data: Vec<u8>,
    dirty: bool,
    position: u64,
}

impl Sector {
    /// Create a new Sector
    pub fn new(position: u64) -> Self {
        Sector {
            data: Vec::<u8>::new(),
            dirty: true,
            position: 0,
        }
    }

    /// Returns the position of the sector on a Disk
    pub fn get_position(&self) -> u64 {
        self.position
    }

    /// Set the position of the Sector on a Disk
    pub fn set_position(&mut self, position: u64) {
        self.position = position;
    }

    /// Marks the Sector as clean
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Marks the Sector as dirty
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Returns true if the sector is dirty, false if it's not.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl Disk {
    /// Instantiate a new Disk struct at a location (Path) and of a certain size in bytes (Size).
    pub fn new(path: &str, mut size: u64) -> Disk {
        size = (size / 512) * 512;
        Disk {
            bootcode: Disk::load_bootcode("DOS622"),
            geometry: Disk::calculate_geometry(size),
            partitions: Vec::<Partition>::new(),
            path: PathBuf::from(path),
            size: size,
            sector_count: size / 512,
            sectors: Vec::<Sector>::new(),
        }
    }
    /// Instantiate an empty Disk struct
    pub fn empty() -> Disk {
        Disk {
            bootcode: Disk::load_bootcode("EMPTY"),
            geometry: CHS::empty(),
            partitions: Vec::<Partition>::new(),
            path: PathBuf::from(""),
            size: 0,
            sector_count: 0,
            sectors: Vec::<Sector>::new(),
        }
    }
    pub fn push_partition(&mut self, partition: Partition) {
        self.partitions.push(partition);
    }
    /// This function loads a specific binary bootcode for use in the Disk struct
    #[allow(unused_assignments)]
    pub fn load_bootcode(os: &str) -> [u8; 446] {
        let mut bootcode: &[u8; 446] = &[0; 446];
        match os {
            "EMPTY" => return *bootcode,
            "DOS622" => bootcode = include_bytes!("os/msdos622-bootcode.bin"),
            &_ => panic!("Invalid bootcode type requested."),
        };
        return *bootcode;
    }

    /// Load a complete Disk struct from an existing file
    pub fn load(path: &str) -> Disk {
        let mut f = OpenOptions::new()
            .read(true)
            .open(path)
            .expect("Failed to open disk image.");
        let mut loaded_disk = Disk::empty();

        // Set the path from the loaded file
        loaded_disk.path = PathBuf::from(path);

        // Set the size from the loaded file
        loaded_disk.size =
            u64::try_from(f.metadata().unwrap().len()).expect("Failed to get file size.");

        // Geometry does not get stored in the image file, so calculate it.
        loaded_disk.geometry = Disk::calculate_geometry(loaded_disk.size);

        // Load existing bootcode from file
        let mut buffer = [0; 446];
        f.read_exact(&mut buffer)
            .expect("Failed to read bootcode from file.");
        loaded_disk.bootcode = buffer;

        return loaded_disk;
    }
    /// Calculate the CHS geometry for a Disk struct based on its size in bytes.
    /// The calculation is based on what the Bochs BIOS expects.
    pub fn calculate_geometry(size: u64) -> CHS {
        // Small disks use the 'none' algorithm
        if size < 528482304 {
            return Disk::geometry_none(size);
        }
        if size < 4227858432 {
            return Disk::geometry_large(size);
        } else {
            panic!("No suitable geometry algorithm available. Disk is probably too big.");
        }
    }
    /// Convert an LBA sector address to a CHS-tuple on a specific disk.
    /// The disk is needed because the calculation depends on the geometry of the underlying disk.
    pub fn lba_to_chs(&self, lba: u32) -> CHS {
        let mut chs = CHS::empty();
        let sectors_per_track = u32::from(self.geometry.sector);
        let heads_per_cylinder = u32::from(self.geometry.head);
        chs.cylinder = u16::try_from(lba / (heads_per_cylinder * sectors_per_track))
            .expect("Too many cylinders!");
        chs.head =
            u8::try_from((lba / sectors_per_track) % heads_per_cylinder).expect("Too many heads!");
        chs.sector = u8::try_from((lba % sectors_per_track) + 1).expect("Too many sectors!");
        return chs;
    }
    /// Convert a CHS-tuple to an LBA sector address.
    #[allow(non_snake_case)]
    pub fn chs_to_lba(&self, sector: &CHS) -> u32 {
        let C = u32::from(sector.cylinder);
        let TH = u32::from(self.geometry.head);
        let TS = u32::from(self.geometry.sector);
        let H = u32::from(sector.head);
        let S = u32::from(sector.sector);
        let lba: u32 = (C * TH * TS) + (H * TS) + (S - 1);
        return lba;
    }
    /// Bochs geomtry algorithm for the 'no translation' case.
    /// Disks that remain within the original int13h limit of 528MB.
    fn geometry_none(size: u64) -> CHS {
        let sector_count = size / 512;
        let mut geom = CHS::empty();
        let heads_range = 1..=15;
        for hpc in heads_range.rev() {
            let cylinders = sector_count / (hpc * 63);
            geom.cylinder = u16::try_from(cylinders).unwrap();
            geom.head = u8::try_from(hpc).unwrap();
            geom.sector = 63;
            if cylinders < 1023 {
                break;
            }
        }
        return geom;
    }
    /// Geometry calculation for disks larger than the LBA limit.
    /// [TODO] This still needs a working implementation!
    fn geometry_large(_size: u64) -> CHS {
        // Empty placeholder for now so this compiles.
        return CHS::empty();
    }
    /// Commit the in-memory Disk struct to persistent storage.
    pub fn write(&self) {
        let f = File::create(self.path.as_path()).expect("Failed to create file.");
        f.set_len(self.size)
            .expect("Failed to grow file to requested size.");
        self.write_bootcode();
        self.write_partitions();
        self.write_signature();
    }
    pub fn write_bytes(&self, offset: u32, bytes: &Vec<u8>) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .expect("Failed to open file.");
        file.seek(SeekFrom::Start(u64::from(offset))).unwrap();
        file.write_all(&bytes).unwrap();
        file.sync_all();
    }
    pub fn write_sector(&self, sector: u64, data: [u8; 512]) {
        if sector > self.sector_count {
            panic!("Sector not available on this disk.");
        }
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .expect("Failed to open disk for writing.");
        file.seek(SeekFrom::Start(u64::from(sector * 512))).unwrap();
        file.write_all(&data).unwrap();
        file.sync_all();
    }
    pub fn read_sector(&self, sector: u64) -> [u8; 512] {
        if sector > self.sector_count {
            panic!("Sector not available on this disk.");
        }
        let file = OpenOptions::new()
            .read(true)
            .open(&self.path)
            .expect("Failed to open disk for reading.");
        let mut reader = BufReader::new(file);
        let mut sector_buffer = [0u8; 512];
        reader
            .seek(SeekFrom::Start(u64::from(sector * 512)))
            .unwrap();
        reader.read_exact(&mut sector_buffer);
        sector_buffer
    }
    /// Write bootcode bytes to the MBR
    pub fn write_bootcode(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .expect("Failed to open file.");
        file.write_all(&self.bootcode).unwrap();
        file.sync_all();
    }
    /// Loop through the Partition structs and commit them to disk.
    pub fn write_partitions(&self) {
        for partition in &self.partitions {
            let mut f = OpenOptions::new()
                .write(true)
                .open(&self.path)
                .expect("Failed to open file.");
            f.seek(SeekFrom::Start(u64::from(partition.offset)))
                .unwrap();
            let bytes = partition.as_bytes();
            f.write_all(&bytes).unwrap();
            f.sync_all();
        }
    }
    /// Write the "magic number" signature bytes to the MBR
    pub fn write_signature(&self) {
        let signature: [u8; 2] = [0x55, 0xAA];
        let mut f = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .expect("Failed to open file.");
        f.seek(SeekFrom::Start(0x1FE)).unwrap();
        f.write_all(&signature).unwrap();
        f.sync_all();
    }
}
