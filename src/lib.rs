use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Seek;
use std::io::prelude::*;
use std::io::SeekFrom;
use bitvec::prelude::*;

mod tests;

#[derive(Debug)]
pub struct Disk {
    bootcode: [u8; 446],
    geometry: CHS,
    pub partitions: Vec<Partition>,
    path: PathBuf,
    size: u64,
}

impl Disk {
    pub fn new(path: &str, size: u64) -> Disk {
        Disk {
            bootcode: Disk::load_bootcode("DOS622"),
            geometry: Disk::calculate_geometry(size),
            partitions: Vec::<Partition>::new(),
            path: PathBuf::from(path),
            size: (size / 512) * 512,
        }
    }
    pub fn empty() -> Disk {
        Disk {
            bootcode: Disk::load_bootcode("EMPTY"),
            geometry: CHS::empty(),
            partitions: Vec::<Partition>::new(),
            path: PathBuf::from(""),
            size: 0,
        }
    }

    #[allow(unused_assignments)]
    pub fn load_bootcode(os: &str) -> [u8; 446] {
        let mut bootcode: &[u8; 446] = &[0; 446];
        match os {
              "EMPTY" => return *bootcode,
              "DOS622" => bootcode = include_bytes!("msdos622-bootcode.bin"),
              &_ => panic!("Invalid bootcode type requested."),
        };
        return *bootcode;
    }

    // Load a complete Disk structure from an existing file
    pub fn load(path: &str) -> Disk {
        let mut f = OpenOptions::new().read(true).open(path).expect("Failed to open disk image.");
        let mut loaded_disk = Disk::empty();

        // Set the path from the loaded file
        loaded_disk.path = PathBuf::from(path);

        // Set the size from the loaded file
        loaded_disk.size = u64::try_from(f.metadata().unwrap().len()).expect("Failed to get file size.");

        // Geometry does not get stored in the image file, so calculate it.
        loaded_disk.geometry = Disk::calculate_geometry(loaded_disk.size);

        // Load existing bootcode from file
        let mut buffer = [0; 446];
        f.read_exact(&mut buffer).expect("Failed to read bootcode from file.");
        loaded_disk.bootcode = buffer;

        return loaded_disk
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
    pub fn lba_to_chs(&self, lba: u32) -> CHS {
        let mut chs = CHS::empty();
        let sectors_per_track = u32::from(self.geometry.sector);
        let heads_per_cylinder = u32::from(self.geometry.head);
        chs.cylinder = u16::try_from(lba / (heads_per_cylinder * sectors_per_track)).expect("Too many cylinders!");
        chs.head = u8::try_from((lba / sectors_per_track) % heads_per_cylinder).expect("Too many heads!");
        chs.sector = u8::try_from((lba % sectors_per_track) + 1).expect("Too many sectors!");
        return chs;
    }
/*    pub fn chs_to_lba(&self, sector: CHS) -> u32 {
        let C = u32::from(sector.cylinder);
        let TH = u32::from(self.geometry.head);
        let TS = u32::from(self.geometry.sector);
        let H = u32::from(sector.head);
        let S = u32::from(sector.sector);
        let lba: u32 = (C * TH * TS) + (H * TS) + (S - 1);
        return lba
    } */
    // Bochs geomtry algorithm for the 'no translation' case.
    // Disks that remain within the original int13h limit of 528MB.
    fn geometry_none(size: u64) -> CHS {
        let sector_count = size / 512;
        let mut geom = CHS::empty();
        let heads_range = 1..=15;
        for hpc in heads_range.rev() {
            let cylinders = sector_count / (hpc * 63);
            geom.cylinder = u16::try_from(cylinders).unwrap();
            geom.head = u8::try_from(hpc).unwrap();
            geom.sector = 63;
            if cylinders < 1023 { break; }
        }
        return geom;
    }
    fn geometry_large(_size: u64) -> CHS {
        // Empty placeholder for now so this compiles.
        return CHS::empty();
    }
    pub fn write(&self) {
        let f = File::create(self.path.as_path()).expect("Failed to create file.");
        f.set_len(self.size).expect("Failed to grow file to requested size.");
        self.write_bootcode();
        self.write_partitions();
        self.write_signature();
    }
    pub fn write_bootcode(&self) {
        let mut file = OpenOptions::new().write(true).open(&self.path).expect("Failed to open file.");
        file.write_all(&self.bootcode).unwrap();
    }
    pub fn write_partitions(&self) {
        for partition in &self.partitions {
            let mut f = OpenOptions::new().write(true).open(&self.path).expect("Failed to open file.");
            f.seek(SeekFrom::Start(u64::from(partition.offset))).unwrap();
            let bytes = partition.as_bytes();
            f.write_all(&bytes).unwrap();
        }
    }
    pub fn write_signature(&self) {
         let signature: [u8; 2] = [0x55, 0xAA];
         let mut f = OpenOptions::new().write(true).open(&self.path).expect("Failed to open file.");
         f.seek(SeekFrom::Start(0x1FE)).unwrap();
         f.write_all(&signature).unwrap();
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
         // Turn the cylinders u16 into a BitVec so we can twiddle bits
         let cylinders_as_bits = BitVec::<_, Msb0>::from_element(self.cylinder);
         let cylinders_clipped = &cylinders_as_bits[6..=15];

         // Turn the sectors u8 into a BitVec so we can twiddle bits
         let sectors_as_bits = BitVec::<_, Msb0>::from_element(self.sector);
         let sectors_clipped = &sectors_as_bits[2..=7];

         let heads_as_bits = BitVec::<_, Msb0>::from_element(self.head);

         // Two variables hold the two distinct parts of the cylinders field
         let cylinders_overflow_bits = &cylinders_clipped[0..=1];
         let cylinders_byte = &cylinders_clipped[2..=9];

         // Create a sequence of 24 bits to gather all the bits in the right sequence
         let mut collected_bits: BitVec::<u32, bitvec::order::Msb0> = BitVec::new();
         collected_bits.extend_from_bitslice(&heads_as_bits);
         collected_bits.extend_from_bitslice(&cylinders_overflow_bits);
         collected_bits.extend_from_bitslice(&sectors_clipped);
         collected_bits.extend_from_bitslice(&cylinders_byte);

         // Compose the sectors field (a byte) from the cylinder overlow bits and the 6 relevant bits from sectors.
         let mut sectors_byte: bitvec::vec::BitVec<_, bitvec::order::Msb0> = BitVec::<u8, bitvec::order::Msb0>::new();
         sectors_byte.extend_from_bitslice(cylinders_overflow_bits);
         sectors_byte.extend_from_bitslice(sectors_clipped);

         // Convert the twiddled fields back to u8's
         let heads_as_u8 = heads_as_bits.load_le::<u8>();
         let sectors_as_u8 = sectors_byte.load_le::<u8>();
         let cylinders_as_u8 = cylinders_byte.load_le::<u8>();

         // ..and return them as an array.
         [heads_as_u8, sectors_as_u8, cylinders_as_u8]
    }
    pub fn from_bytes(bytes: [u8; 3]) -> CHS {
        // Turn the bytes into sequences of bits
        let heads_byte = BitVec::<_, Msb0>::from_element(bytes[0]);
        let sectors_byte = BitVec::<_, Msb0>::from_element(bytes[1]);
        let cylinders_byte = BitVec::<_, Msb0>::from_element(bytes[2]);

        // Put all those bits together into a Vec that's 24 bits long
        let mut chs_bits: BitVec::<u8, bitvec::order::Msb0> = BitVec::new();
        chs_bits.extend_from_bitslice(&heads_byte);
        chs_bits.extend_from_bitslice(&sectors_byte);
        chs_bits.extend_from_bitslice(&cylinders_byte);

        // The heads byte comes over unmodified, it's the first 8 bits of the sequence
        let mut chs_heads: BitVec::<u8, bitvec::order::Msb0> = BitVec::new();
        chs_heads.extend_from_bitslice(&chs_bits[0..=7]);

        // The sectors number is only 6 bits long, so pad it with zeroes to bring it up to 8 bits.
        let mut chs_sectors: BitVec::<u8, bitvec::order::Msb0> = BitVec::new();
        chs_sectors.extend_from_bitslice(&chs_bits[9..=15]);

        // The cylinders value is 10 bits long but a u16 has room for 16, so pad with zeroes.
        let mut chs_cylinders: BitVec::<u16, bitvec::order::Msb0> = BitVec::new();
        let mut i = 0;
        while i < 6 {
            chs_cylinders.push(false);
            i +=1;
        }
        // Pull in the two overflow bits that were stored in the sectors byte
        chs_cylinders.extend_from_bitslice(&chs_bits[8..=9]);
        // Finish off by adding the actual cylinders byte itself
        chs_cylinders.extend_from_bitslice(&chs_bits[16..=23]);

        // Convert all these bits back to their numerical types and create the CHS struct
        let mut chs = CHS::empty();
        chs.head = chs_heads.load_le::<u8>();
        chs.sector = chs_sectors.load_le::<u8>();
        chs.cylinder = chs_cylinders.load_le::<u16>();
        return chs;
    }
}

#[derive(Debug)]
pub struct Partition {
    offset: u16,
    flag_byte: u8,
    first_lba: u32,
    first_sector: CHS,
    partition_type: u8,
    last_sector: CHS,
    sector_count: u32,
}

impl Partition {
    pub fn new(disk: &Disk, partition_number: u8, mut start_sector: u32, partsize: u64) -> Partition {
        let size: u32 = u32::try_from(partsize).unwrap();
        // See if we could ever fit at all: panic early if we don't.
        let max_size: u32 = u32::try_from(disk.size - (63 * 512)).unwrap();
        if size > max_size {
            panic!("Requested partition won't fit on your drive.");
        }

        // MBR layout doesn't support more than 4 primary partitions. Extended partitioning is out of scope (for now).
        if partition_number > 4 || partition_number == 0 {
            panic!("Can't have more than 4 partitions, starting at offset 1. You tried to create one at offset {}", partition_number);
        }

        // Can't have things begin before sector 63. Theoretically, sure, but MS-DOS doesn't do it that way.
        if start_sector < 63 {
            start_sector = 63
        }

        // Check if we can really fit the disk
        let sector_count = size - start_sector;
        let requested_sectors = size / 512;
        if requested_sectors > sector_count {
            panic!("The requested partion won't fit on your drive. If possible, have it start in another place on the drive.");
        }
        println!("Requested sectors: {:?}", requested_sectors);
        let my_partition = Partition {
            offset: 0x1be,
            flag_byte: 0x80,
            first_sector: disk.lba_to_chs(start_sector),
            partition_type: 0x06,
            last_sector: disk.lba_to_chs(requested_sectors - start_sector),
            first_lba: start_sector,
            sector_count: sector_count,
        };
        println!("Generated partition: {:?}", my_partition);
        return my_partition;
    }
    pub fn as_bytes(&self) -> Vec::<u8> {
        let mut bytes = Vec::<u8>::new();
        let start_chs = self.first_sector.as_bytes();
        let end_chs = self.last_sector.as_bytes();
        bytes.push(self.flag_byte);
        for byte in start_chs {
            bytes.push(byte);
        }
        bytes.push(self.partition_type);
        for byte in end_chs {
            bytes.push(byte);
        }
        for byte in self.first_lba.to_le_bytes() {
            bytes.push(byte);
        }
        for byte in self.sector_count.to_le_bytes() {
            bytes.push(byte);
        }
        return bytes;
    }
}
