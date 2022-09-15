use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use bitvec::prelude::*;

#[derive(Debug)]
pub struct Disk {
    bootcode: [u8; 446],
    geometry: CHS,
    path: PathBuf,
    size: u64,
}

impl Disk {
    pub fn new(path: &str, size: u64) -> Disk {
        let bootcode = include_bytes!("msdos622-bootcode.bin");
        Disk {
            bootcode: *bootcode,
            geometry: Disk::calculate_geometry(size),
            path: PathBuf::from(path),
            size: size,
        }
    }
    pub fn calculate_geometry(size: u64) -> CHS {
        // Small disks use the 'none' algorithm
        if size < 528482304 {
            return Disk::geometry_none(size);
        }
        if size < 4227858432 {
            return Disk::geometry_large(size);
        }
        else {
            panic!("No suitable geometry algorithm available. Disk is probably too big.");
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
        return geom;
    }
    fn geometry_large(size: u64) -> CHS {
        // Empty placeholder for now so this compiles.
        return CHS::empty();
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
    // Calculate triplet of CHS bytes for use in partition tables
    // https://thestarman.pcministry.com/asm/mbr/PartTables.htm#mbr
    pub fn as_bytes(&self) -> [u8; 3] {
        // Handle the simple case: values all fit in their own bytes.
        if self.cylinder <= 255 {
            return [
                u8::try_from(self.cylinder).unwrap(),
                u8::try_from(self.head).unwrap(),
                u8::try_from(self.sector).unwrap(), ]
        }
        // The cylinders value can go to 1024, which needs 10 bits.
        // I don't know how to do the bit-twiddling yet to make this
        // fit across the 3 bytes that the MBR permits for this.
        else {
             // Turn the cylinders u16 into a BitVec so we can twiddle bits
             let mut cylinders_as_bits = BitVec::<_, Msb0>::from_element(self.cylinder);
             cylinders_as_bits.drain(0..=5); // Clip off the first 6 unwanted bits

             // Turn the sectors u8 into a BitVec so we can twiddle bits
             let mut sectors_as_bits = BitVec::<_, Msb0>::from_element(self.sector);
             sectors_as_bits.drain(0..=1); // Clip off the two bytes to make room for the bits of overflow from cylinder

             // Two variables hold the two distinct parts of the cylinders field
             let cylinders_overflow_bits = &cylinders_as_bits[0..=1];
             let cylinders_byte = &cylinders_as_bits[1..=8];

             // Compose the sectors field (a byte) from the cylinder overlow bits and the 6 relevant bits from sectors.
             let mut sectors_byte = bitvec![];
             sectors_byte.extend_from_bitslice(cylinders_overflow_bits);
             sectors_byte.extend_from_bitslice(&sectors_as_bits);

             // Convert the twiddled fields back to u8's
             let sectors_as_u8 = sectors_byte.load_le::<u8>();
             let cylinders_as_u8 = cylinders_byte.load_le::<u8>();

             // ..and return them as an array.
             // We don't process the heads field because that's already a byte that doesn't need to be touched.
             [cylinders_as_u8, self.head, sectors_as_u8]
        }
    }
}
