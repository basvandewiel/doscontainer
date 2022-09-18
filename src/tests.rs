#[cfg(test)]
mod tests {
    use crate::Disk;
    use crate::Partition;
    use crate::CHS;

    #[test]
    fn disk_geometry() {
        let mut my_disk = Disk::new("test.raw", 5000000);
        my_disk.partitions.push(Partition::new(1, CHS::from_bytes([0x1f, 0x3f, 0x33]), 5000000));
        my_disk.write();
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
