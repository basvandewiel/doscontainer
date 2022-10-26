use crate::dks::chs::CHS;
use crate::partition::Partition;
use crate::sector::Sector;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::*;
use std::path::PathBuf;

pub mod chs;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Disk {
    bootcode: [u8; 446],
    geometry: CHS,
    partitions: Vec<Partition>,
    path: PathBuf,
    size: usize,
    sector_count: usize,
    sectors: Vec<Sector>,
}

impl Disk {
    /// Create a new Disk struct
    pub fn new(path: &str, size: usize, operating_system: &str) -> Disk {
        let final_size = (size / 512) * 512;
        Disk {
            bootcode: Disk::load_bootcode(operating_system),
            geometry: Disk::calculate_geometry(final_size),
            partitions: Vec::<Partition>::with_capacity(4),
            path: PathBuf::from(path),
            size: final_size,
            sector_count: size / 512,
            sectors: Vec::<Sector>::with_capacity(size / 512),
        }
    }

    /// Add a partition to the Disk. Position must be an integer between 1 and 4.
    pub fn add_partition(&mut self, partition: Partition, u8: position) {
        if position > 3 {
            panic!("A Disk can only hold a maximum of 4 partitions.");
        }
        if position < 1 {
            panic!("Disk partition numbers start from 1.");
        }
        // Ensure the location for our partition is available
        if self.partitions.size() < position {
            self.partitions.resize(position + 1);
        }
        self.partitions[position] = partition;
    }

    /// Load the bootcode for a particular operating system
    pub fn load_bootcode(os: &str) -> [u8; 446] {
        let mut bootcode: &[u8; 446] = &[0; 446];
        match os {
            "EMPTY" => return *bootcode,
            "DOS622" => bootcode = include_bytes!("../os/msdos622-bootcode.bin"),
            &_ => panic!("Invalid bootcode type requested."),
        };
        return *bootcode;
    }

    /// Calculate CHS geometry
    pub fn calculate_geometry(size: usize) -> CHS {
        if size < 528482304 {
            return Disk::geometry_none(size);
        } else {
            panic!("No suitable geometry algorithm available. Disk is probably too big.");
        }
    }

    // Bochs geometry algorithm for the 'no translation' case.
    fn geometry_none(size: usize) -> CHS {
        let sector_count = size / 512;
        let mut geom = CHS::empty();
        let heads_range = 1..=16;
        for hpc in heads_range.rev() {
            let cylinders = sector_count / (hpc * 63);
            geom.cylinder = u16::try_from(cylinders).expect("Too many cylinders!");
            geom.head = u8::try_from(hpc).expect("Too many heads!");
            geom.sector = 63;
            if cylinders < 1023 {
                break;
            }
        }
        return geom;
    }
}
