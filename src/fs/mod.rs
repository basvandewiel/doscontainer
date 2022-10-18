// use crate::fs::fat::FAT;
use crate::fs::vbr::VBR;
use crate::fs::cluster::Cluster;
use bitvec::prelude::*;

mod tests;
pub mod vbr;
mod cluster;
pub mod fat;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct File {
    name: String,
    data: Vec<u8>,
    clusters: Vec<Cluster>,
    attributes: FileAttributes,
}

impl File {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        File {
            name: name,
            data: data,
            clusters: Vec::<Cluster>::new(),
            attributes: FileAttributes::default(),
        }
    }
    pub fn set_readonly(&mut self, readonly: bool) {
        self.attributes.read_only = readonly;
    }
    pub fn set_hidden(&mut self, hidden: bool) {
        self.attributes.hidden = hidden;
    }
    pub fn set_system(&mut self, system: bool) {
        self.attributes.system = system;
    }
    pub fn set_vol_id(&mut self, vol_id: bool) {
        self.attributes.vol_id = vol_id;
    }
    pub fn set_is_dir(&mut self, is_dir: bool) {
        self.attributes.is_dir = is_dir;
    }
    pub fn set_archive(&mut self, archive: bool) {
        self.attributes.archive = archive;
    }
    pub fn get_size(&self) -> usize {
        return self.data.len();
    }
    /// Validate the requested file name according to the
    /// rules as determined by Microsoft in their spec document
    /// on page 24.
    /// [TODO] Use a proper Result<E, T> and expand so this function
    /// doesn't just validate but can serve to normalize a filename.
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
