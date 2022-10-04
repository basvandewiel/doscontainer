use crate::fs::vbr::VBR;
use bitvec::prelude::*;

mod tests;
pub mod vbr;

#[derive(Debug)]
pub struct FAT {
    files: Vec<File>,
    sector_count: u32,
    sectors_per_fat: u32,
    clusters: Vec<u16>,
    cluster_count: u32,
    cluster_size: usize,
}

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
    clusters: Vec<u16>,
    attributes: FileAttributes,
}

impl File {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        File {
            name: name,
            data: data,
            clusters: Vec::<u16>::new(),
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

#[derive(Debug)]
pub struct Directory {}

impl FAT {
    /// Instantiate a new FAT struct based on sector count
    pub fn new(sector_count: u32) -> Self {
        FAT {
            files: Vec::<File>::new(),
            sector_count: u32::from(VBR::set_sectors_per_fat(sector_count)),
            clusters: FAT::initialize_fat(
                (sector_count / u32::from(VBR::set_sectors_per_cluster(sector_count)))
                    .try_into()
                    .unwrap(),
            ),
            cluster_count: sector_count / u32::from(VBR::set_sectors_per_cluster(sector_count)),
            cluster_size: usize::from(VBR::set_sectors_per_cluster(sector_count)) * 512,
            sectors_per_fat: 46,
        }
    }

    /// No idea why this is there yet. Cluster 0 contains this when formatted
    /// using MS-DOS so I'm replicating it here.
    fn initialize_fat(cluster_count: usize) -> Vec<u16> {
        let mut clusters = vec![0; cluster_count];
        clusters[0] = 0xfff8;
        clusters
    }

    /// Push a new file onto the file system.
    pub fn push_file(&self, mut file: File) {
        file.clusters = self.allocate_clusters(&file);
        let chunks = file.data.chunks(self.cluster_size);
    }

    /// Return a list of free clusters for use by the given File
    /// We're regenerating the whole disk with every write, so we always get
    /// perfect defragmentation and race conditions don't exit.
    pub fn allocate_clusters(&self, file: &File) -> Vec<u16> {
        let filesize: usize = file.get_size(); // Size of file in bytes
        let mut required_clusters = 0usize;
        if filesize < self.cluster_size {
            required_clusters = 1;
        } else {
            required_clusters = num::integer::div_ceil(filesize, self.cluster_size) + 1;
        }
        let mut free_clusters = Vec::<u16>::new();
        for (i, item) in self
            .clusters
            .iter()
            .enumerate()
            .take(required_clusters.try_into().unwrap())
        {
            if *item == 0x0000 {
                free_clusters.push(i as u16);
            }
        }
        free_clusters
    }
}
