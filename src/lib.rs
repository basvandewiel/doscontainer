use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
pub struct Disk {
    bootcode: [u8; 446],
    geometry: CHS,
    path: PathBuf,
    sector_count: u64,
    sector_size: u64,
    size: u64,
}

impl Disk {
    pub fn new(path: &str, size: u64) -> Disk {
        if size > 528482304 { panic!("Disk too big!"); }
        let sector_size = 512;
        let bootcode = include_bytes!("msdos622-bootcode.bin");
        Disk {
            bootcode: *bootcode,
            geometry: Disk::geometry_none(size),
            path: PathBuf::from(path),
            sector_count: size / sector_size,
            sector_size: 512,
            size: size,
        }
    }
    // Bochs geomtry algorithm for the 'no translation' case.
    // Disks that remain within the original int13h limit of 528MB.
    fn geometry_none(size: u64) -> CHS {
        let sector_count = size / 512;
        let mut geom = CHS::empty();
        let heads_range = 1..=16;
        for hpc in heads_range {
            let cylinders = sector_count / (hpc * 63);
            geom.cylinder = u16::try_from(cylinders).unwrap();
            geom.head = u8::try_from(hpc).unwrap();
            geom.sector = 63;
            if cylinders < 1023 { break; }
        }
        println!("{:?}", geom);
        return geom;
    }
    pub fn write(&self) {
        let f = File::create(self.path.as_path()).expect("Failed to create file.");
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
