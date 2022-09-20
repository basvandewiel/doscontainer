pub mod fs {
    // Volume Boot Record: the boot sector placed at the start of a partition
    #[derive(Debug)]
    pub struct VBR {
        jumpbytes: [u8; 3],
        oem_name: [u8; 8],
        bios_parameter_block: BiosParameterBlock,
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
        
        // Setter for input sanitation
        pub fn set_sectors_per_cluster(&mut self, sectors_per_cluster: u8) {
            let valid_values = Vec::<u8>::from([1, 2, 4, 8, 16, 32, 64, 128]);
            if valid_values.contains(&sectors_per_cluster) {
                self.sectors_per_cluster = sectors_per_cluster;
            }
            else {
                panic!("Got an invalid value for sectors per cluster.");
            }
        }
    }
}
