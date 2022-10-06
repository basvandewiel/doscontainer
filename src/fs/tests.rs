use crate::fs::File;
use crate::fs::FileAttributes;
use crate::fs::FAT;
use crate::fs::VBR;

#[test]
pub fn attributes_empty() {
    let attribs = FileAttributes::default();
    assert_eq!(attribs.as_byte(), 0);
}

#[test]
pub fn attribute_readonly() {
    let mut attribs = FileAttributes::default();
    attribs.read_only = true;
    assert_eq!(attribs.as_byte(), 1);
}

#[test]
pub fn attribute_hidden() {
    let mut attribs = FileAttributes::default();
    attribs.hidden = true;
    assert_eq!(attribs.as_byte(), 2);
}

#[test]
pub fn attribute_system() {
    let mut attribs = FileAttributes::default();
    attribs.system = true;
    assert_eq!(attribs.as_byte(), 4);
}

#[test]
pub fn attribute_vol_id() {
    let mut attribs = FileAttributes::default();
    attribs.vol_id = true;
    assert_eq!(attribs.as_byte(), 8);
}

#[test]
pub fn attribute_is_dir() {
    let mut attribs = FileAttributes::default();
    attribs.is_dir = true;
    assert_eq!(attribs.as_byte(), 16);
}

#[test]
pub fn attribute_archive() {
    let mut attribs = FileAttributes::default();
    attribs.archive = true;
    assert_eq!(attribs.as_byte(), 32);
}

#[test]
/// Special combined case for MS-DOS system files
pub fn attribs_rsh_enabled() {
    let mut attribs = FileAttributes::default();
    attribs.read_only = true;
    attribs.hidden = true;
    attribs.system = true;
    assert_eq!(attribs.as_byte(), 7);
}

#[test]
pub fn valid_filename() {
    let name: &str = "FILEEXE";
    assert_eq!(File::validate_name(name), true);
}

#[test]
pub fn invalid_filename_starts_with_space() {
    let name: &str = " FILEEXE";
    assert_eq!(File::validate_name(name), false);
}

#[test]
pub fn invalid_filename_has_dotchar() {
    let name: &str = "FILE.EXE";
    assert_eq!(File::validate_name(name), false);
}

#[test]
pub fn invalide_filename_too_long() {
    let name: &str = "THISFILENAMEISMUCHTOOLONG";
    assert_eq!(File::validate_name(name), false);
}

#[test]
pub fn fat_cluster_count() {
    let fat = FAT::new(94532);
    assert_eq!(fat.cluster_count, 23633);
    assert_eq!(fat.clusters.len(), 23633);
}

#[test]
#[should_panic]
pub fn fat_cluster_count_too_big() {
    let fat = FAT::new(99999999);
}

#[test]
pub fn fat_cluster_size() {
    let fat = FAT::new(94532);
    assert_eq!(fat.cluster_size, 2048);
}

#[test]
pub fn vbr_as_bytes() {
    let vbr = VBR::new(94532);
    let reference: Vec<u8> = vec![
        235, 60, 144, 77, 83, 68, 79, 83, 53, 46, 48, 0, 2, 4, 1, 0, 2, 0, 2, 0, 0, 248, 93, 0, 63,
        0, 16, 0, 63, 0, 0, 0, 68, 113, 1, 0, 128, 0, 41, 241, 202, 53, 99, 68, 79, 83, 67, 78, 84,
        78, 82, 32, 32, 32, 70, 65, 84, 49, 54, 32, 32, 32, 250, 51, 192, 142, 208, 188, 0, 124,
        22, 7, 187, 120, 0, 54, 197, 55, 30, 86, 22, 83, 191, 62, 124, 185, 11, 0, 252, 243, 164,
        6, 31, 198, 69, 254, 15, 139, 14, 24, 124, 136, 77, 249, 137, 71, 2, 199, 7, 62, 124, 251,
        205, 19, 114, 121, 51, 192, 57, 6, 19, 124, 116, 8, 139, 14, 19, 124, 137, 14, 32, 124,
        160, 16, 124, 247, 38, 22, 124, 3, 6, 28, 124, 19, 22, 30, 124, 3, 6, 14, 124, 131, 210, 0,
        163, 80, 124, 137, 22, 82, 124, 163, 73, 124, 137, 22, 75, 124, 184, 32, 0, 247, 38, 17,
        124, 139, 30, 11, 124, 3, 195, 72, 247, 243, 1, 6, 73, 124, 131, 22, 75, 124, 0, 187, 0, 5,
        139, 22, 82, 124, 161, 80, 124, 232, 146, 0, 114, 29, 176, 1, 232, 172, 0, 114, 22, 139,
        251, 185, 11, 0, 190, 230, 125, 243, 166, 117, 10, 141, 127, 32, 185, 11, 0, 243, 166, 116,
        24, 190, 158, 125, 232, 95, 0, 51, 192, 205, 22, 94, 31, 143, 4, 143, 68, 2, 205, 25, 88,
        88, 88, 235, 232, 139, 71, 26, 72, 72, 138, 30, 13, 124, 50, 255, 247, 227, 3, 6, 73, 124,
        19, 22, 75, 124, 187, 0, 7, 185, 3, 0, 80, 82, 81, 232, 58, 0, 114, 216, 176, 1, 232, 84,
        0, 89, 90, 88, 114, 187, 5, 1, 0, 131, 210, 0, 3, 30, 11, 124, 226, 226, 138, 46, 21, 124,
        138, 22, 36, 124, 139, 30, 73, 124, 161, 75, 124, 234, 0, 0, 112, 0, 172, 10, 192, 116, 41,
        180, 14, 187, 7, 0, 205, 16, 235, 242, 59, 22, 24, 124, 115, 25, 247, 54, 24, 124, 254,
        194, 136, 22, 79, 124, 51, 210, 247, 54, 26, 124, 136, 22, 37, 124, 163, 77, 124, 248, 195,
        249, 195, 180, 2, 139, 22, 77, 124, 177, 6, 210, 230, 10, 54, 79, 124, 139, 202, 134, 233,
        138, 22, 36, 124, 138, 54, 37, 124, 205, 19, 195, 13, 10, 78, 111, 110, 45, 83, 121, 115,
        116, 101, 109, 32, 100, 105, 115, 107, 32, 111, 114, 32, 100, 105, 115, 107, 32, 101, 114,
        114, 111, 114, 13, 10, 82, 101, 112, 108, 97, 99, 101, 32, 97, 110, 100, 32, 112, 114, 101,
        115, 115, 32, 97, 110, 121, 32, 107, 101, 121, 32, 119, 104, 101, 110, 32, 114, 101, 97,
        100, 121, 13, 10, 0, 73, 79, 32, 32, 32, 32, 32, 32, 83, 89, 83, 77, 83, 68, 79, 83, 32,
        32, 32, 83, 89, 83, 0,
    ];
    assert_eq!(vbr.as_bytes(), reference);
}

#[test]
pub fn iosys_to_clusters() {
    let io_sys = File::new("IOSYS".to_string(), include_bytes!("../os/IO.SYS").to_vec());
    let fat = FAT::new(94532);
    let reference: Vec<u16> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let clusters = fat.allocate_clusters(&io_sys);
    assert_eq!(clusters, reference);
}

#[test]
pub fn msdossys_to_clusters() {
    let msdos_sys = File::new(
        "MSDOSSYS".to_string(),
        include_bytes!("../os/MSDOS.SYS").to_vec(),
    );
    let fat = FAT::new(94532);
    let reference: Vec<u16> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ];
    let clusters = fat.allocate_clusters(&msdos_sys);
    assert_eq!(clusters, reference);
}
