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
                bios_parameter_block: BiosParameterBlock::new(partition),
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
        // Return an immutable ref to the BIOS Parameter Block
        pub fn get_bpb(&self) -> &BiosParameterBlock {
            return &self.bios_parameter_block;
        }
    }

    /* MS-DOS 4.0+ BIOS Parameter Block (Extended Parameter Block).
     * This derives from the MS-DOS 3.31 BPB with a few added fields.
     * What we're essentially implementing here, is FAT16B.
     * See: https://en.wikipedia.org/wiki/File_Allocation_Table#FAT16
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
        hidden_sectors_count: u32,
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
            // Start with empty, base the other values on the partition we get passed in
            let mut bpb = BiosParameterBlock::empty();
            bpb.set_bytes_per_sector(512); // Hard-coded default
            bpb.set_sectors_per_cluster(bpb.calculate_sectors_per_cluster(partition));
            bpb.set_reserved_sectors(1);
            bpb.set_number_of_fats(2);
            bpb.set_number_of_root_entries(512);
            if partition.sector_count < 65536 {
                bpb.set_sector_count(u16::try_from(partition.sector_count).unwrap());
                bpb.total_sectors_count = 0; // MS FAT spec page 8
            }
            else {
                bpb.set_sector_count(0);
                bpb.total_sectors_count = partition.sector_count;
            }
            bpb.set_media_descriptor(0xF8);
            bpb.set_sectors_per_fat(bpb.calculate_sectors_per_fat(partition));
            return bpb;
        }
        pub fn as_bytes(&self) -> Vec::<u8> {
            let mut bytes = Vec::<u8>::new();
            bytes.push(1u8);
            return bytes;
        }
        // Follow the table MS FAT16 spec, page 13, up to 1GB.
        fn calculate_sectors_per_cluster(&self, partition: &Partition) -> u8 {
            let megabyte: u32 = 1024*1024; // Define a megabyte
            let partition_size = (partition.sector_count * 512)/megabyte; // Partition size in megabytes
            if partition_size > 512 {
                return 32;
            }
            if partition_size > 256 {
                return 16;
            }
            if partition_size > 128 {
                return 8;
            }
            if partition_size > 16 {
                return 4;
            }
            else {
                return 2;
            }
        }
        // Calculate sectors per FAT according to Microsoft spec page 14.
        // According to Microsoft this algorithm sucks, but it fails in a
        // safe way and is therefore acceptable. Same old Microsoft, but
        // since it's impossible to change the past.. this is our future.
        fn calculate_sectors_per_fat(&self, partition: &Partition) -> u16 {
            let rootdir_sectors = ((self.number_of_root_entries * 32) + 511) / 512;
            let tmpval1: u32 = partition.sector_count - (u32::from(self.get_reserved_sectors()) + u32::from(rootdir_sectors));
            let tmpval2: u32 = (256 * u32::from(self.get_sectors_per_cluster())) + u32::from(self.get_number_of_fats());
            let fat_size: u32 = (tmpval1 + (tmpval2 -1)) / tmpval2;
            return u16::try_from(fat_size).expect("Value too large for FAT16");
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
                // 0xF8 is the standard value for HDD's
                self.media_descriptor = 0xF8;
            }
        }
        
        pub fn get_media_descriptor(&self) -> u8 {
            return self.media_descriptor;
        }

        pub fn set_sectors_per_fat(&mut self, sectors_per_fat: u16) {
            self.sectors_per_fat = sectors_per_fat;
        }

        pub fn get_sectors_per_fat(&self) -> u16 {
            return self.sectors_per_fat;
        }

        pub fn set_disk_sectors_per_track(&mut self, disk_sectors_per_track: u16) {
            let valid_values = Vec::<u16>::from([63, 127, 255]);
            if valid_values.contains(&disk_sectors_per_track) {
                self.disk_sectors_per_track = disk_sectors_per_track;
            }
            // Educated guess: if this goes wrong, just use 63 instead of panic
            else {
                self.disk_sectors_per_track = 63;
            }
        }

        pub fn get_disk_sectors_per_track(&self) -> u16 {
            return self.disk_sectors_per_track;
        }

        pub fn set_disk_heads(&mut self, mut disk_heads: u16) {
            // Avoid division by zero errors in software that doesn't expect a zero here
            // If your software doesn't use CHS anyway, then this field being 1 should be neutral.
            if disk_heads == 0 {
                disk_heads = 1;
            }
            // While the variable is a u16, the maximum value is 255
            if disk_heads > 255 {
                disk_heads = 255;
            }
            self.disk_heads = disk_heads;
        }

        pub fn get_disk_heads(&self) -> u16 {
            return self.disk_heads;
        }

        // Thihs should always be zero unless there's a realistic use case
        // Still take a parameter so it doesn't confuse consumers of the function.
        pub fn set_hidden_sectors_count(&mut self, _hidden_sectors_count: u32) {
            self.hidden_sectors_count = 0;
        }

        pub fn get_hidden_sectors_count(&self) -> u32 {
            return self.hidden_sectors_count;
        }
    }
}
