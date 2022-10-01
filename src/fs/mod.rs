use crate::fs::vbr::VBR;
use bitvec::prelude::*;

pub mod vbr;
mod tests;

#[derive(Debug)]
pub struct FAT {
    files: Vec<File>,
    sector_count: u32,
    sectors_per_fat: u32,
    cluster_count: u32,
    cluster_size: u32,
}

#[derive(Debug)]
pub struct FileAttributes {
    read_only: bool,
    hidden: bool,
    system: bool,
    vol_id: bool,
    is_dir: bool,
    archive: bool,
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
