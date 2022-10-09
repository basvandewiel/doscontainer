#[cfg(test)]
mod tests {
    use crate::chs::CHS;
    use crate::disk::Disk;
    use crate::partition::Partition;
    use std::fs;
    use std::path::PathBuf;
    use std::{thread, time};

    #[test]
    fn disk_geometry() {
        let mut my_disk = Disk::new("test.raw", 5000000);
        my_disk
            .partitions
            .push(Partition::new(&my_disk, 1, 63, 4900000));
        my_disk.write();
    }

    #[test]
    #[should_panic]
    fn disk_too_big() {
        let mut my_disk = Disk::new("testdummy", 600000000000);
        my_disk.partitions.push(Partition::new(&my_disk, 1, 63, 0));
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

    // Create an empty disk. This will contain a few bytes that are non-zero.
    // Read them back and compare to the expected result. This cleans up after itself.
    // Uses an excessively unlikely filename so as not to clobber something already there.
    #[test]
    fn read_valid_sector() {
        let mut my_disk = Disk::empty();
        my_disk.path =
            PathBuf::from("ff665c8ce7f5e1585ba2dcdc4109be56ef82dd0fccb5038449cc4fcf178345c1.raw");
        my_disk.size = 50000000;
        my_disk.write();
        // let ten_millis = time::Duration::from_millis(1000);
        // let now = time::Instant::now();
        // thread::sleep(ten_millis);
        let mut reference = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 170,
        ];
        let null_sector = Disk::read_sector(&my_disk, 0);
        fs::remove_file("ff665c8ce7f5e1585ba2dcdc4109be56ef82dd0fccb5038449cc4fcf178345c1.raw");
        assert_eq!(null_sector, reference);
    }

    #[test]
    #[should_panic]
    fn read_sector_out_of_bounds() {
        let mut my_disk = Disk::new("ff665c8ce7f5e1585ba2dcdc4109be56ef82dd0fccb5038449cc4fcf178345c1.raw", 50000000);
        let mut reference = [0u8; 512];
        let bad_sector = Disk::read_sector(&my_disk, 2000000000);
        assert_eq!(bad_sector, reference);
    }

    // Create new disk. Write a single byte to sector 2, location 20. Read back the sector and check if we get
    // that single byte back.
    #[test]
    fn write_valid_sector() {
        let mut my_disk = Disk::new("af665c8ce7f5e1585ba2dcdc4109be56ef82dd0fccb5038449cc4fcf178345c1.raw", 50000000);
        my_disk.write();
        let mut reference = [0u8; 512];
        reference[20] = 0xFF;
        my_disk.write_sector(2, reference);
        let read_back = my_disk.read_sector(2);
        fs::remove_file("af665c8ce7f5e1585ba2dcdc4109be56ef82dd0fccb5038449cc4fcf178345c1.raw");
        assert_eq!(reference, read_back);
    }
}
