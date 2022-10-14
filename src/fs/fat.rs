use crate::fs::Cluster;
use crate::fs::File;
use crate::fs::VBR;

#[derive(Debug)]
pub struct FAT {
    files: Vec<File>,
    sector_count: u32,
    sectors_per_fat: u32,
    clusters: Vec<Cluster>,
    cluster_count: u32,
    cluster_size: usize,
}

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

        // Loop over the clusters in this FAT to find any that are marked as 0x0000 (unallocated).
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
