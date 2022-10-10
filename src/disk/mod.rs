use crate::chs::CHS;
use crate::fs::vbr::VBR;
use crate::partition::Partition;
use crate::sector::Sector;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::*;
use std::path::PathBuf;

mod tests;

/// A Disk is the holding structure for a collection of Sectors. It also
/// represents the interface between what the emulator gets to see, and what is
/// present as a file on the host computer.
#[derive(Debug)]
pub struct Disk {
    pub(crate) bootcode: [u8; 446],
    pub(crate) geometry: CHS,
    pub(crate) partitions: Vec<Partition>,
    pub(crate) path: PathBuf,
    pub(crate) size: usize,
    pub(crate) sector_count: usize,
    sectors: Vec<Sector>,
}

impl Disk {
    /// Instantiate a new Disk struct at a location (Path) and of a certain size in bytes (Size).
    pub fn new(path: &str, mut size: usize) -> Disk {
        // Fudge the provided size so that it gets sector-aligned
        size = (size / 512) * 512;
        // Compose the final Disk struct to return
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

    /// Retrieve a Sector struct reference from this instance of Disk.
    pub fn get_sector(&self, position: usize) -> &Sector {
        &self.sectors[position]
    }

    /// Push a Partition struct into this Disk's partition table
    pub fn push_partition(&mut self, partition: Partition) {
        self.partitions.push(partition);
    }

    /// This function loads a specific binary bootcode for use in the Disk struct
    #[allow(unused_assignments)]
    pub fn load_bootcode(os: &str) -> [u8; 446] {
        let mut bootcode: &[u8; 446] = &[0; 446];
        match os {
            "EMPTY" => return *bootcode,
            "DOS622" => bootcode = include_bytes!("../os/msdos622-bootcode.bin"),
            &_ => panic!("Invalid bootcode type requested."),
        };
        return *bootcode;
    }

    /// Add a sector to the end of the current Disk structure
    pub fn push_sector(&mut self, sector: Sector) {
        self.sectors.push(sector);
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
        loaded_disk.size = usize::try_from(f.metadata().unwrap().len())
            .expect("Failed to get size from disk image.");

        // Test if size is a multiple of 512, making it sector-aligned
        if (loaded_disk.size / 512) * 512 != loaded_disk.size {
            panic!("Disk image is not sector aligned.");
        }

        loaded_disk.sector_count = loaded_disk.size / 512;

        // Geometry does not get stored in the image file, so calculate it.
        loaded_disk.geometry = Disk::calculate_geometry(loaded_disk.size);

        // Load the Sectors from Disk
        for position in 0..loaded_disk.sector_count {
            let sector = loaded_disk.read_sector(position);
            loaded_disk.push_sector(sector);
        }

        // Load existing bootcode from file
        let mut buffer = [0; 446];
        f.read_exact(&mut buffer)
            .expect("Failed to read bootcode from file.");
        loaded_disk.bootcode = buffer;

        return loaded_disk;
    }

    /// Calculate the CHS geometry for a Disk struct based on its size in bytes.
    /// The calculation is based on what the Bochs BIOS expects.
    pub fn calculate_geometry(size: usize) -> CHS {
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
    fn geometry_none(size: usize) -> CHS {
        let sector_count = size / 512;
        let mut geom = CHS::empty();
        let heads_range = 1..=15;
        for hpc in heads_range.rev() {
            let cylinders = sector_count / (hpc * 63);
            geom.cylinder = u16::try_from(cylinders).expect("Too many cylinders!");
            geom.head = u8::try_from(hpc).expect("Too many heads!");
            geom.sector = 63;
            if cylinders < 1023 {
                break;
            }
        }
        return geom;
    }

    /// Geometry calculation for disks larger than the LBA limit.
    /// [TODO] This still needs a working implementation!
    fn geometry_large(_size: usize) -> CHS {
        // Empty placeholder for now so this compiles.
        return CHS::empty();
    }

    /// Commit the in-memory Disk struct to persistent storage.
    pub fn write(&self) {
        let f = File::create(self.path.as_path()).expect("Failed to create file.");
        f.set_len(u64::try_from(self.size).unwrap())
            .expect("Failed to grow file to requested size.");
    }

    /// Generate a valid MBR boot sector and put it into this Disk's sector 0.
    pub fn build_bootsector(&mut self) {
        let mut bootsector = Sector::new(0);
        // Walk through the bootcode bytes and place them at the start of the sector.
        // This (usually) is x86 assembly code that was lifted straight from the original OS.
        for (index, byte) in self.bootcode.iter().enumerate() {
            bootsector.write_byte(index, *byte);
        }
        // Walk through Partition structs in the Disk object and add table entries for them.
        for partition in &self.partitions {
            let bytes = partition.as_bytes();
            let mut counter = partition.offset;
            for byte in bytes {
                bootsector.write_byte(counter.into(), byte);
                counter += 1;
            }
        }
        // A valid DOS MBR boot sector requires two "magic" bytes at the very end. These are the
        // values and we simply put them there because that's how the world works.
        bootsector.write_byte(0x1FE, 0x55);
        bootsector.write_byte(0x1FF, 0xAA);
        if self.sectors.len() <= 0 {
            self.sectors.push(bootsector);
        } else {
            self.sectors[0] = bootsector;
        }
    }

    pub fn write_bytes(&self, offset: u32, bytes: &Vec<u8>) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .expect("Failed to open file.");
        file.seek(SeekFrom::Start(u64::from(offset))).unwrap();
        file.write_all(&bytes).unwrap();
        file.sync_all().unwrap();
    }

    /// Read a Sector struct from persistent storage
    pub fn read_sector(&self, sector: usize) -> Sector {
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
            .seek(SeekFrom::Start(
                u64::try_from(sector * 512)
                    .expect("Failed to seek to requested sector's position."),
            ))
            .unwrap();
        reader.read_exact(&mut sector_buffer);
        let mut new_sector = Sector::new(sector);
        for (index, byte) in sector_buffer.iter().enumerate() {
            new_sector.write_byte(index, *byte);
        }
        new_sector
    }
}
