use crate::Partition;

pub enum FatType {
    Fat12,
    Fat16,
}

impl FatType {
    // Type of FAT depends on the number of clusters in a volume
    pub fn from_clusters(cluster_count: u32) -> Self {
        if cluster_count < 4085 {
            FatType::Fat12
        } else {
            FatType::Fat16
        }
    }
}

#[derive(Debug)]
pub struct FAT {
    sectors_per_cluster: u8,
    cluster_count: u16,
    entries: Vec<u16>,
}

impl FAT {
    pub fn new(partition: &Partition) -> FAT {
        let bpb = partition.vbr.as_ref().expect("").get_bpb();
        let cluster_count = partition.sector_count / u32::from(bpb.get_sectors_per_cluster());

        FAT {
            sectors_per_cluster: bpb.get_sectors_per_cluster(),
            cluster_count: u16::try_from(cluster_count).unwrap(),
            entries: Vec::<u16>::new(),
        }
    }
    pub fn get_cluster_count(&self) -> u16 {
        return self.cluster_count;
    }
}
