use crate::Partition;

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
