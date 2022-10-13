use crate::disk::Disk;
use crate::disk::CHS;
use crate::partition::Partition;
use crate::sector::Sector;
use std::fs;
use std::path::PathBuf;

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

/// This should test disk geometry calculations but doesn't yet.
#[test]
fn disk_geometry() {
    let mut my_disk = Disk::new("asdf4qfawfd23rwfdasdf23rrgasdf.raw", 5000000);
    my_disk.push_partition(Partition::new(&my_disk, 1, 63, 4900000));
    my_disk.write();
    fs::remove_file("asdf4qfawfd23rwfdasdf23rrgasdf.raw").unwrap();
}

/// Test failure mode for creating a disk that is (much) too big.
#[test]
#[should_panic]
fn disk_too_big() {
    let mut my_disk = Disk::new("testdummy", 600000000000);
    my_disk.partitions.push(Partition::new(&my_disk, 1, 63, 0));
}

/// Request an empty bootcode
#[test]
fn request_empty_bootcode() {
    let bootcode: [u8; 446] = Disk::load_bootcode("EMPTY");
    assert_eq!(bootcode, [0; 446]);
}

/// Request the MS-DOS 6.22 bootcode, compare length and a few bytes.
#[test]
fn request_msdos622_bootcode() {
    let bootcode: [u8; 446] = Disk::load_bootcode("DOS622");
    assert_eq!(bootcode.len(), 446);
    assert_eq!(bootcode[0], 250);
    assert_eq!(bootcode[217], 109);
    assert_eq!(bootcode[218], 0);
}

/// Request a wrong type of bootcode
#[test]
#[should_panic]
fn request_wrong_bootcode() {
    Disk::load_bootcode("wrong_file.bin");
}

/// Create an empty disk. This will contain a few bytes that are non-zero.
/// Read them back and compare to the expected result. This cleans up after itself.
/// Uses an excessively unlikely filename so as not to clobber something already there.
#[test]
fn read_valid_sector() {
    let mut my_disk = Disk::empty();
    my_disk.path =
        PathBuf::from("ff665c8ce7f5e1585ba2dcdc4109be56ef82dd0fccb5038449cc4fcf178345c1.raw");
    my_disk.size = 50000000;
    my_disk.write();
    let mut reference: [u8; 512] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let mut reference_sector = Sector::new(0);
    for (index, byte) in reference.iter().enumerate() {
        reference_sector.write_byte(index, *byte);
    }
    let null_sector = my_disk.read_sector(0);
    fs::remove_file("ff665c8ce7f5e1585ba2dcdc4109be56ef82dd0fccb5038449cc4fcf178345c1.raw")
        .unwrap();
    assert_eq!(null_sector, reference_sector);
}

/// Test for failure when trying to read a sector beyond the edge of a Disk.
#[test]
#[should_panic]
fn read_sector_out_of_bounds() {
    let my_disk = Disk::new(
        "ff665c8ce7f5e1585ba2dcdc4109be56ef82dd0fccb5038449cc4fcf178345c1.raw",
        50000000,
    );
    my_disk.write();
    let reference = Sector::new(0);
    let bad_sector = Disk::read_sector(&my_disk, 2000000000);
    assert_eq!(bad_sector, reference);
}

/// Test building a Sector struct for the bootsector. Compare the default MS-DOS 6.22 boot sector to the
/// statically provided byte-array in this function
#[test]
fn disk_build_bootsector() {
    let mut my_disk = Disk::new("asdf24r2asdf2erasfasd2rafd.raw", 50000000);
    my_disk.push_partition(Partition::new(&my_disk, 1, 63, 50000000));
    let reference_data: [u8; 512] = [
        250, 51, 192, 142, 208, 188, 0, 124, 139, 244, 80, 7, 80, 31, 251, 252, 191, 0, 6, 185, 0,
        1, 242, 165, 234, 29, 6, 0, 0, 190, 190, 7, 179, 4, 128, 60, 128, 116, 14, 128, 60, 0, 117,
        28, 131, 198, 16, 254, 203, 117, 239, 205, 24, 139, 20, 139, 76, 2, 139, 238, 131, 198, 16,
        254, 203, 116, 26, 128, 60, 0, 116, 244, 190, 139, 6, 172, 60, 0, 116, 11, 86, 187, 7, 0,
        180, 14, 205, 16, 94, 235, 240, 235, 254, 191, 5, 0, 187, 0, 124, 184, 1, 2, 87, 205, 19,
        95, 115, 12, 51, 192, 205, 19, 79, 117, 237, 190, 163, 6, 235, 211, 190, 194, 6, 191, 254,
        125, 129, 61, 85, 170, 117, 199, 139, 245, 234, 0, 124, 0, 0, 73, 110, 118, 97, 108, 105,
        100, 32, 112, 97, 114, 116, 105, 116, 105, 111, 110, 32, 116, 97, 98, 108, 101, 0, 69, 114,
        114, 111, 114, 32, 108, 111, 97, 100, 105, 110, 103, 32, 111, 112, 101, 114, 97, 116, 105,
        110, 103, 32, 115, 121, 115, 116, 101, 109, 0, 77, 105, 115, 115, 105, 110, 103, 32, 111,
        112, 101, 114, 97, 116, 105, 110, 103, 32, 115, 121, 115, 116, 101, 109, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 1, 1, 0, 6, 15, 63, 101, 63, 0, 0, 0, 196, 120, 1,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 170,
    ];
    let mut reference_sector = Sector::new(0);
    for (index, byte) in reference_data.iter().enumerate() {
        reference_sector.write_byte(index, *byte);
    }
    my_disk.build_bootsector();
    let mut bootsector = my_disk.get_sector(0);
    assert_eq!(bootsector, &reference_sector);
}
