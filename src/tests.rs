#[cfg(test)]
mod tests {
    use crate::Disk;
    use crate::Partition;
    use crate::CHS;
    use crate::fs::fs::*;

    #[test]
    fn disk_geometry() {
        let mut my_disk = Disk::new("test.raw", 5000000);
        my_disk.partitions.push(Partition::new(&my_disk, 1, 63, 4900000));
        my_disk.write();
    }

    // Create a partition
    #[test]
    fn create_partition() {
        let my_disk = Disk::new("test.raw", 50000000);
        let my_partition = Partition::new(&my_disk, 1, 63, 49000000);
        assert_eq!(my_partition.offset, 446);
        assert_eq!(my_partition.flag_byte, 128);
        assert_eq!(my_partition.first_lba, 63);
        assert_eq!(my_partition.first_sector.cylinder, 0);
        assert_eq!(my_partition.first_sector.head, 1);
        assert_eq!(my_partition.first_sector.sector, 1);
        assert_eq!(my_partition.partition_type, 6);
        assert_eq!(my_partition.last_sector.cylinder, 99);
        assert_eq!(my_partition.last_sector.head, 15);
        assert_eq!(my_partition.last_sector.sector, 63);
        assert_eq!(my_partition.sector_count, 94562);
    }

    // Request an empty bootcode
    #[test]
    fn request_empty_bootcode() {
        let bootcode: [u8; 446] = Disk::load_bootcode("EMPTY");
        assert_eq!(bootcode, [0; 446]);
    }

    // Request the MS-DOS 6.22 bootcode, compare length and a few bytes.
    #[test]
    fn request_msdos622_bootcode() {
        let bootcode: [u8; 446] = Disk::load_bootcode("DOS622");
        assert_eq!(bootcode.len(), 446);
        assert_eq!(bootcode[0], 250);
        assert_eq!(bootcode[217], 109);
        assert_eq!(bootcode[218], 0);
    }

    // Request a wrong type of bootcode
    #[test]
    #[should_panic]
    fn request_wrong_bootcode() {
        Disk::load_bootcode("wrong_file.bin");
    }

    // Generate the correct set of bytes from a CHS struct
    #[test]
    fn chs_to_bytes() {
        let mut chs = CHS::empty();
        chs.head = 254;
        chs.sector = 63;
        chs.cylinder = 723;
        let bytes: [u8; 3] = chs.as_bytes();
        assert_eq!(bytes[0], 0xfe);
        assert_eq!(bytes[1], 0xbf);
        assert_eq!(bytes[2], 0xd3);
    }

    // Generate a correct CHS struct from a set of bytes
    #[test]
    fn chs_from_bytes() {
        let chs = CHS::from_bytes([0xfe, 0xbf, 0xd3]);
        assert_eq!(chs.head, 254);
        assert_eq!(chs.sector, 63);
        assert_eq!(chs.cylinder, 723);
    }

    // FS setters
    #[test]
    fn bpb_set_too_few_bytes_per_sector() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_bytes_per_sector(12);
        assert_eq!(bpb.get_bytes_per_sector(), 32);
    }

    #[test]
    fn bpb_set_too_many_bytes_per_sector() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_bytes_per_sector(32768);
        assert_eq!(bpb.get_bytes_per_sector(), 4096);
    }

    #[test]
    fn bpb_set_sensible_bytes_per_sector() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_bytes_per_sector(2048);
        assert_eq!(bpb.get_bytes_per_sector(), 2048);
    }

    #[test]
    fn bpb_set_too_few_sectors_per_cluster() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_sectors_per_cluster(0);
        assert_eq!(bpb.get_sectors_per_cluster(), 8);
    }

    #[test]
    fn bpb_set_valid_sectors_per_cluster() {
        let valid_values: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
        let mut bpb = BiosParameterBlock::empty();
        for value in valid_values {
            bpb.set_sectors_per_cluster(value);
            assert_eq!(bpb.get_sectors_per_cluster(), value);
        }
    }

    #[test]
    fn bpb_set_too_many_sectors_per_cluster() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_sectors_per_cluster(255);
        assert_eq!(bpb.get_sectors_per_cluster(), 8);
    }

    #[test]
    fn bpb_set_too_few_reserved_sectors() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_reserved_sectors(0);
        assert_eq!(bpb.get_reserved_sectors(), 1);
    }

    #[test]
    fn bpb_set_valid_reserved_sectors() {
        let mut bpb = BiosParameterBlock::empty();
        for value in 1..65535 {
            bpb.set_reserved_sectors(value);
            assert_eq!(bpb.get_reserved_sectors(), value);
        }
    }

    #[test]
    fn bpb_set_too_few_root_entries() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_number_of_root_entries(5);
        assert_eq!(bpb.get_number_of_root_entries(), 16);
    }

    #[test]
    fn bpb_set_wrong_number_of_root_entries() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_number_of_root_entries(241);
        assert_eq!(bpb.get_number_of_root_entries(), 240);
    }

    #[test]
    fn bpb_set_too_many_root_entries() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_number_of_root_entries(4000);
        assert_eq!(bpb.get_number_of_root_entries(), 512);
    }

    #[test]
    fn bpb_set_too_few_sectors() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_sector_count(3);
        assert_eq!(bpb.get_sector_count(), 64);
    }

    #[test]
    fn bpb_set_valid_number_of_sectors() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_sector_count(128);
        assert_eq!(bpb.get_sector_count(), 128);
    }

    #[test]
    fn bpb_set_valid_media_descriptor() {
        let valid_values: [u8; 15] = [0xe5, 0xed, 0xee, 0xef, 0xf0, 0xf4, 0xf5, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff];
        let mut bpb = BiosParameterBlock::empty();
        for value in valid_values {
            bpb.set_media_descriptor(value);
            assert_eq!(bpb.get_media_descriptor(), value);
        }
    }

    #[test]
    #[should_panic]
    fn bpb_set_invalid_media_descriptor() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_media_descriptor(0xb3);
    }

    #[test]
    fn bpb_sectors_per_fat() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_sectors_per_fat(32);
        assert_eq!(bpb.get_sectors_per_fat(), 32);
    }

    #[test]
    fn bpb_set_valid_disk_sectors_per_track() {
        let mut bpb = BiosParameterBlock::empty();
        let valid_values: [u16; 3] = [63, 127, 255];
        for value in valid_values {
            bpb.set_disk_sectors_per_track(value);
            assert_eq!(bpb.get_disk_sectors_per_track(), value);
        }
    }

    #[test]
    fn bpb_set_invalid_disk_sectors_per_track() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_disk_sectors_per_track(24);
        assert_eq!(bpb.get_disk_sectors_per_track(), 63);
    }

    #[test]
    fn bpb_set_zero_disk_heads() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_disk_heads(0);
        assert_eq!(bpb.get_disk_heads(), 1);
    }

    #[test]
    fn bpb_set_too_many_disk_heads() {
        let mut bpb = BiosParameterBlock::empty();
        bpb.set_disk_heads(600);
        assert_eq!(bpb.get_disk_heads(), 255);
    }

    #[test]
    fn bpb_hidden_sectors_always_zero() {
        let mut bpb = BiosParameterBlock::empty();
        // Limit the interval to something comfortably above the max. number of sectors we support anyway.
        // Running this test for the full scope of a u32 takes unnecessarily long.
        for value in 0..70000 {
            bpb.set_hidden_sectors_count(value);
            assert_eq!(bpb.get_hidden_sectors_count(), 0);
        }

    fn bpb_as_bytes() {
        let mut bpb = BiosParameterBlock::empty();
        assert_eq!(bpb.as_bytes()[0], 1);
    }

    #[test]
    fn vbr_jumpbytes() {
        let disk = Disk::new("testdummy", 50000000);
        let part = Partition::new(&disk, 1, 63, 0);
        let vbr = VBR::new(part);
        assert_eq!(vbr.get_jumpbytes(), [0xeb, 0x3c, 0x90]);
    }

    #[test]
    fn vbr_oemname() {
        let disk = Disk::new("testdummy", 50000000);
        let part = Partition::new(&disk, 1, 63, 0);
        let vbr = VBR::new(part);
        assert_eq!(vbr.get_oem_name(), [0x4D, 0x53, 0x44, 0x4F, 0x53, 0x35, 0x2E, 0x30]);
    }

    #[test]
    fn vbr_as_bytes() {
        let disk = Disk::new("testdummy", 50000000);
        let part = Partition::new(&disk, 1, 63, 0);
        let vbr = VBR::new(part);
        let bytes: Vec::<u8> = [0xEB, 0x3C, 0x90, 0x4D, 0x53, 0x44, 0x4F, 0x53, 0x35, 0x2E, 0x30, 0x01].to_vec();
        assert_eq!(vbr.as_bytes(), bytes);
    }
}
