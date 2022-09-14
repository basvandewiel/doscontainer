use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
pub struct Disk {
    path: PathBuf,
    size: u64,
    sector_count: u64,
    bootcode: [u8; 446],
}

impl Disk {
    pub fn new(path: &str, size: u64) -> Disk {
        let sector_size = 512;
        let bootcode = include_bytes!("msdos622-bootcode.bin");
        Disk {
            path: PathBuf::from(path),
            size: size,
            sector_count: size / sector_size,
            bootcode: *bootcode,
        }
    }
    pub fn write(&self) {
        let mut f = File::create(self.path.as_path()).expect("Failed to create file.");
        f.set_len(self.size).expect("Failed to grow file to requested size.");
    }
    pub fn write_bootcode(&self) {
        let mut file = OpenOptions::new().write(true).open(&self.path).expect("Failed to open file.");
        file.write_all(&self.bootcode).unwrap();
    }
}

// Custom type for Cylinder/Head/Sector geometry
#[derive(Debug)]
pub struct CHS {
    cylinder: u16,
    head: u8,
    sector: u8,
}

impl CHS {
    pub fn empty() -> CHS {
        CHS {
            cylinder: 0,
            head: 0,
            sector: 0
        }
    }
}
