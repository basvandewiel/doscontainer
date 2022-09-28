pub struct VBR {
    jump_bytes: [u8; 3],
    oem_name: [u8; 8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors_count: u16,
    fats_count: u8,
    root_dir_entries_count: u16,
    sectors_per_fat: u16,
    media_descriptor: u8,
    sectors_per_track: u16,
    heads_count: u16,
    hidden_sectors_count: u16,
    volume_boot_code: Vec<u8>,
}

impl VBR {
    /// Instantiate a new Volume Boot Record struct.
    pub fn new(volume_sector_count: u32) -> Self {
        VBR {
            jump_bytes: VBR::default_jump_bytes(),
            oem_name: VBR::default_oem_name(),
            bytes_per_sector: 512, // Hardcoded default for the MiSTer use case
            sectors_per_cluster: VBR::set_sectors_per_cluster(volume_sector_count),
            reserved_sectors_count: 1, // Hardcoded default for ancient MS/PC-DOS
            fats_count: 2,             // Hardcoded default for ancient MS/PC-DOS
            root_dir_entries_count: 512, // See MS FAT32 Spec page 8 for rationale.
            sectors_per_fat: VBR::set_sectors_per_fat(volume_sector_count),
            media_descriptor: 0xF8, // Default for hard disks. We don't support floppies.
            sectors_per_track: 63,  // Read from an MS-DOS VBR
            heads_count: 16,        // Read from an MS-DOS VBR
            hidden_sectors_count: 63, // Read from an MS-DOS VBR
            volume_boot_code: include_bytes!("os/msdos622-vbr-bootcode.bin").to_vec(),
        }
    }

    /// Serialize a Volume Boot Record struct into
    /// a sequence of bytes suitable for the on-disk format
    pub fn as_bytes(&self) -> Vec<u8> {
        let bytes = Vec::<u8>::new();
        return bytes;
    }

    pub fn get_bootcode(&self) -> &Vec<u8> {
        return &self.volume_boot_code;
    }

    /// Jump to the bootstrap routine. These are three
    /// x86 machine language instructions that constitute
    /// a jump into the machine language routine that's
    /// stored elsewhere in the same sector. The most common
    /// value found here is 0xEB 0x3C 0x90, the latter of which
    /// is a NOP. The value depends on the operating system. This
    /// function returns the default for MS-DOS 6.x for now.
    fn default_jump_bytes() -> [u8; 3] {
        return [0xEB, 0x3C, 0x90];
    }

    /// MS-DOS 6.x interestingly uses MSDOS5.0 as the OEM name.
    fn default_oem_name() -> [u8; 8] {
        return *b"MSDOS5.0";
    }

    /// Set the Sectors per Cluster value according to Microsoft specs,
    /// see page 13 of the official FAT32 Spec for the values used in FAT16.
    /// This is a non-zero power of 2 that must fit within a single byte.
    /// The number depends on the size of the partition in sectors.
    fn set_sectors_per_cluster(volume_sector_count: u32) -> u8 {
        if volume_sector_count < 8400 {
            panic!("Less than 8400 sectors is an error condition for FAT16");
        } else if volume_sector_count < 32680 {
            return 2;
        } else if volume_sector_count < 262144 {
            return 4;
        } else if volume_sector_count < 524288 {
            return 8;
        } else if volume_sector_count < 1048576 {
            return 16;
        } else if volume_sector_count < 2097152 {
            return 32;
        } else if volume_sector_count < 4194304 {
            return 64;
        } else {
            panic!("Over 4194304 sectors is an error condition for FAT16");
        }
    }

    /// The sectors per FAT value is calculated according to an algorithm
    /// provided by Microsoft in the FAT32 Spec document on page 14. It is
    /// fundamentally flawed but it's what MS apparently used in their OS'es
    /// so for accuracy we replicate it here. Pass in the partition as a reference
    /// so we can calculate the values instead of depending on a populated &self.
    fn set_sectors_per_fat(volume_sector_count: u32) -> u16 {
        let root_dir_sectors = ((512 * 32) + (512 - 1)) / 512;
        let tmpval1 = volume_sector_count - (1 + root_dir_sectors);
        let tmpval2: u32 = (256 * u32::from(VBR::set_sectors_per_cluster(volume_sector_count))) + 2;
        let fat_size = (tmpval1 + (u32::from(tmpval2) - 1)) / u32::from(tmpval2);
        if fat_size < 65535 {
            return u16::try_from(fat_size).unwrap();
        } else {
            panic!("Number of sectors per FAT too large for BPB");
        }
    }
}
