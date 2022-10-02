use crate::fs::vbr::VBR;
use bitvec::prelude::*;

mod tests;
pub mod vbr;

#[derive(Debug)]
pub struct FAT {
    files: Vec<File>,
    sector_count: u32,
    sectors_per_fat: u32,
    cluster_count: u32,
    cluster_size: u32,
}

#[derive(Debug)]
/// Attributes that can be set on a file or directory
/// in a FAT file system.
pub struct FileAttributes {
    pub(crate) read_only: bool,
    pub(crate) hidden: bool,
    pub(crate) system: bool,
    pub(crate) vol_id: bool,
    pub(crate) is_dir: bool,
    pub(crate) archive: bool,
}

impl Default for FileAttributes {
    fn default() -> FileAttributes {
        FileAttributes {
            read_only: false,
            hidden: false,
            system: false,
            vol_id: false,
            is_dir: false,
            archive: false,
        }
    }
}

impl FileAttributes {
    /// Convert the attributes to a single byte
    /// that can be used directly on-disk.
    pub fn as_byte(&self) -> u8 {
        let mut data = 0u8;
        let bits = data.view_bits_mut::<Lsb0>();
        bits.set(0, self.read_only);
        bits.set(1, self.hidden);
        bits.set(2, self.system);
        bits.set(3, self.vol_id);
        bits.set(4, self.is_dir);
        bits.set(5, self.archive);
        data
    }
}

#[derive(Debug)]
pub struct File {
    name: [u8; 8],
    extension: [u8; 3],
    data: Vec<u8>,
    attributes: FileAttributes,
}

impl File {
    /// Validate the requested file name according to the
    /// rules as determined by Microsoft in their spec document
    /// on page 24.
    fn validate_name(name: &str) -> bool {
        // Name must not be longer than 11 characters
        if name.len() > 11 {
            return false;
        }
        let bytes = name.as_bytes();

        // Special validation required for the first character
        if bytes[0] == 0x00 {
            return false;
        }
        if bytes[0] == 0x20 {
            return false;
        }

        // Check for disallowed characters
        let invalid_chars: [u8; 16] = [
            0x22, 0x2A, 0x2B, 0x2C, 0x2E, 0x2F, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x5B, 0x5C,
            0x5D, 0x7C,
        ];
        for value in bytes {
            if invalid_chars.contains(&value) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Directory {}

impl FAT {
    /// Instantiate a new FAT struct based on sector count
    pub fn new(sector_count: u32) -> Self {
        FAT {
            files: Vec::<File>::new(),
            sector_count: u32::from(VBR::set_sectors_per_fat(sector_count)),
            cluster_count: sector_count / u32::from(VBR::set_sectors_per_cluster(sector_count)),
            cluster_size: u32::from(u32::from(VBR::set_sectors_per_cluster(sector_count)) * 512),
            sectors_per_fat: 46,
        }
    }

    pub fn add_file(file: File) {}
}

impl File {}
