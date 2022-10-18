use crate::disk::Disk;
use crate::partition::Partition;

#[test]
fn partition_roundtrip() {
    let mut disk = Disk::new("bogus_test_file.raw", 50000000);
    let partition = Partition::new(&disk, 1, 63, 0);
    let bytes = partition.as_bytes();
    let mut bytes_array = [0u8; 16];
    for (i, byte) in bytes.iter().enumerate() {
        bytes_array[i] = *byte;
    }
    let reconstituted_partition = Partition::from_bytes(bytes_array);
    assert_eq!(partition.offset, reconstituted_partition.offset);
    assert_eq!(partition.flag_byte, reconstituted_partition.flag_byte);
    assert_eq!(partition.first_lba, reconstituted_partition.first_lba);
    assert_eq!(partition.first_sector, reconstituted_partition.first_sector);
    assert_eq!(
        partition.partition_type,
        reconstituted_partition.partition_type
    );
    assert_eq!(partition.last_sector, reconstituted_partition.last_sector);
    assert_eq!(partition.sector_count, reconstituted_partition.sector_count);
    assert_eq!(partition.last_lba, reconstituted_partition.last_lba);
    assert_eq!(partition.boot_record, reconstituted_partition.boot_record);
    assert_eq!(partition.FAT, reconstituted_partition.FAT);
}

#[test]
fn partition__100mb_roundtrip() {
    let mut disk = Disk::new("bogus_test_file.raw", 100000000);
    let partition = Partition::new(&disk, 1, 63, 0);
    let bytes = partition.as_bytes();
    let mut bytes_array = [0u8; 16];
    for (i, byte) in bytes.iter().enumerate() {
        bytes_array[i] = *byte;
    }
    let reconstituted_partition = Partition::from_bytes(bytes_array);
    assert_eq!(partition.offset, reconstituted_partition.offset);
    assert_eq!(partition.flag_byte, reconstituted_partition.flag_byte);
    assert_eq!(partition.first_lba, reconstituted_partition.first_lba);
    assert_eq!(partition.first_sector, reconstituted_partition.first_sector);
    assert_eq!(
        partition.partition_type,
        reconstituted_partition.partition_type
    );
    assert_eq!(partition.last_sector, reconstituted_partition.last_sector);
    assert_eq!(partition.sector_count, reconstituted_partition.sector_count);
    assert_eq!(partition.last_lba, reconstituted_partition.last_lba);
    assert_eq!(partition.boot_record, reconstituted_partition.boot_record);
    assert_eq!(partition.FAT, reconstituted_partition.FAT);
}
