use crate::chs::CHS;
use crate::partition::Partition;
use fatfs::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Disk {
    pub(crate) bootcode: [u8; 446],
    pub(crate) geometry: CHS,
    pub(crate) partitions: Vec<Partition>,
    pub(crate) path: PathBuf,
    pub(crate) size: u64,
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
        }
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
        self.format_partition(&self.partitions[0]);
        self.write_sys(&self.partitions[0]);
    }
    pub fn format_partition(&self, partition: &Partition) {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .unwrap();
        let file_part = fscommon::StreamSlice::new(
            file,
            partition.get_start_offset(),
            partition.get_end_offset(),
        )
        .unwrap();
        fatfs::format_volume(file_part, FormatVolumeOptions::new()).unwrap();
    }
    pub fn write_sys(&self, partition: &Partition) {
        // Integrate the bytes for MS-DOS system files
        let io_sys = include_bytes!("os/IO.SYS");
        let msdos_sys = include_bytes!("os/MSDOS.SYS");
        let command_com = include_bytes!("os/COMMAND.COM");

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .unwrap();
        let file_part = fscommon::StreamSlice::new(
            file,
            partition.get_start_offset(),
            partition.get_end_offset(),
        )
        .unwrap();
        let options = fatfs::FsOptions::new().update_accessed_date(true);
        let fs = fatfs::FileSystem::new(file_part, options).unwrap();
        let mut iosys = fs.root_dir().create_file("IO.SYS").unwrap();
        iosys.write_all(io_sys).unwrap();
        let mut msdossys = fs.root_dir().create_file("MSDOS.SYS").unwrap();
        msdossys.write_all(msdos_sys).unwrap();
        let mut commandcom = fs.root_dir().create_file("COMMAND.COM").unwrap();
        commandcom.write_all(command_com).unwrap();
    }
    pub fn write_bytes(&self, offset: u32, bytes: &Vec<u8>) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .expect("Failed to open file.");
        file.seek(SeekFrom::Start(u64::from(offset))).unwrap();
        file.write_all(&bytes).unwrap();
    }
    /// Write bootcode bytes to the MBR
    pub fn write_bootcode(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .expect("Failed to open file.");
        file.write_all(&self.bootcode).unwrap();
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
    }
}
