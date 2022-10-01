use crate::fs::FileAttributes;

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
