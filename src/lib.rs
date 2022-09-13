use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Disk {
    path: PathBuf,
    size: u64,
}

impl Disk {
    pub fn new(path: &str, size: u64) -> Disk {
        Disk {
            path: PathBuf::from(path),
            size: size,
        }
    }
    pub fn write(&self) {
        let mut f = File::create(self.path.as_path()).expect("Failed to create file.");
        f.set_len(self.size).expect("Failed to grow file to requested size.");
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
