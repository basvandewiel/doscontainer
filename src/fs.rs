pub mod fs {
    use crate::Partition;

    // Volume Boot Record: the boot sector placed at the start of a partition
    #[derive(Debug)]
    pub struct VBR {
        jumpbytes: [u8; 3],
        oem_name: [u8; 8],
        bios_parameter_block: BiosParameterBlock,
    }

    impl VBR {
        pub fn new(partition: &Partition) -> VBR {
            VBR {
                jumpbytes: [0xEB, 0x3C, 0x90], // MS-DOS 6.22 default jumpbytes
                oem_name: [0x4D, 0x53, 0x44, 0x4F, 0x53, 0x35, 0x2E, 0x30], // MSDOS5.0
                bios_parameter_block: BiosParameterBlock::empty(),
            }
        }
        pub fn as_bytes(&self) -> Vec<u8> {
            // The bytes vector will contain the entire VBR
            let mut bytes = Vec::<u8>::new();

            // Push the individual bytes in sequence as specified by the FAT spec
            for value in self.jumpbytes {
                bytes.push(value);
            }
            for value in self.oem_name {
                bytes.push(value);
            }
            for value in self.bios_parameter_block.as_bytes() {
                bytes.push(value);
            }
            return bytes;
        }
        pub fn get_jumpbytes(&self) -> [u8; 3] {
            return self.jumpbytes;
        }
        pub fn get_oem_name(&self) -> [u8; 8] {
            return self.oem_name;
        }
    }

    /* MS-DOS 4.0+ BIOS Parameter Block (Extended Parameter Block).
     * This derives from the MS-DOS 3.31 BPB with a few added fields.
     */
    #[derive(Debug)]
    pub struct BiosParameterBlock {
        bytes_per_sector: u16,
        sectors_per_cluster: u8,
        reserved_sectors: u16,
        number_of_fats: u8,
        number_of_root_entries: u16,
        sector_count: u16,
        media_descriptor: u8,
        sectors_per_fat: u16,
        disk_sectors_per_track: u16,
        disk_heads: u16,
        hidden_sectors_count: u16,
        total_sectors_count: u32, 
        physical_drive_number: u8, // EBPB starts here
        extended_boot_signature: u8,
        volume_id: u32,
        volume_label: [u8; 11],
        filesystem_type: [u8; 8],
    }
    
    impl BiosParameterBlock {
        // Generate an empty BPB
        pub fn empty() -> BiosParameterBlock {
            BiosParameterBlock {
                bytes_per_sector: 512, // For MiSTer this should be hardcoded.
                sectors_per_cluster: 0,
                reserved_sectors: 1,
                number_of_fats: 2,
                number_of_root_entries: 512,
                sector_count: 0,
                media_descriptor: 0,
                sectors_per_fat: 0,
                disk_sectors_per_track: 0,
                disk_heads: 0,
                hidden_sectors_count: 0,
                total_sectors_count: 0,
                physical_drive_number: 0,
                extended_boot_signature: 0,
                volume_id: 0,
                volume_label: [0; 11],
                filesystem_type: [0; 8],
            }
        }
        pub fn new(partition: &Partition) -> BiosParameterBlock {
            // Keep the build happy for now
            return BiosParameterBlock::empty();
        }
        pub fn as_bytes(&self) -> Vec::<u8> {
            let mut bytes = Vec::<u8>::new();
            bytes.push(1u8);
            return bytes;
        }
        // Setter for input sanitation
        pub fn set_bytes_per_sector(&mut self, mut bytes_per_sector: u16) {
            // Don't support values below 32
            if bytes_per_sector < 32 {
                bytes_per_sector = 32;
            }
            // Even though recent Linux will do 32K, we target pre-2000 systems.
            if bytes_per_sector > 4096 {
                bytes_per_sector = 4096;
            }
            self.bytes_per_sector = bytes_per_sector;
        }
        
        pub fn get_bytes_per_sector(&self) -> u16 {
            return self.bytes_per_sector;
        }
        
        // Setter for input sanitation
        pub fn set_sectors_per_cluster(&mut self, sectors_per_cluster: u8) {
            let valid_values = Vec::<u8>::from([1, 2, 4, 8, 16, 32, 64, 128]);
            if valid_values.contains(&sectors_per_cluster) {
                self.sectors_per_cluster = sectors_per_cluster;
            }
            else {
                // Use the maximum supported by MS-DOS
                self.sectors_per_cluster = 8;
            }
        }
        
        pub fn get_sectors_per_cluster(&self) -> u8 {
            return self.sectors_per_cluster;
        }
        
        // Setter for input sanitation
        pub fn set_reserved_sectors(&mut self, mut reserved_sectors: u16) {
            // 0 is invalid: must reserve the first logical sector
            if reserved_sectors < 1 {
                reserved_sectors = 1;
            }
            self.reserved_sectors = reserved_sectors;
        }
        
        pub fn get_reserved_sectors(&self) -> u16 {
            return self.reserved_sectors;
        }

        // Setter for input sanitation        
        pub fn set_number_of_fats(&mut self, mut number_of_fats: u8) {
            // 0 is invalid, correct it to 1
            if number_of_fats < 1 {
                number_of_fats = 1;
            }
            self.number_of_fats = number_of_fats;
        }
        
        pub fn get_number_of_fats(&self) -> u8 {
            return self.number_of_fats;
        }
        
        // Setter for input sanitation
        pub fn set_number_of_root_entries(&mut self, mut number_of_root_entries: u16) {
            // Correct too small values
            if number_of_root_entries < 16 {
                number_of_root_entries = 16;
            }
            // MS-DOS does not support more than 512 entries on a hard drive
            if number_of_root_entries > 512 {
                number_of_root_entries = 512;
            }
            // Number must always be a multiple of 16
            number_of_root_entries = (number_of_root_entries / 16) * 16;
            self.number_of_root_entries = number_of_root_entries;
        }
        
        pub fn get_number_of_root_entries(&self) -> u16 {
            return self.number_of_root_entries;
        }
        
        pub fn set_sector_count(&mut self, mut sector_count: u16) {
            // Do something sensible here for a minimum number of sectors.
            if sector_count < 64 {
                sector_count = 64;
            }
            self.sector_count = sector_count;
        }
        
        pub fn get_sector_count(&self) -> u16 {
            return self.sector_count;
        }
        
        pub fn set_media_descriptor(&mut self, media_descriptor: u8) {
            let valid_values = Vec::<u8>::from([0xe5, 0xed, 0xee, 0xef, 0xf0, 0xf4, 0xf5, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]);
            if valid_values.contains(&media_descriptor) {
                self.media_descriptor = media_descriptor;
            }
            else {
                panic!("Invalid media descriptor byte.");
            }
        }
        
        pub fn get_media_descriptor(&self) -> u8 {
            return self.media_descriptor;
        }
    }
}
