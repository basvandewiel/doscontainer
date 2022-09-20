#[cfg(test)]
mod tests {
    use crate::Disk;
    use crate::Partition;
    use crate::CHS;

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
}
