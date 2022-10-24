use crate::disk::chs::*;
use crate::disk::*;
use crate::fs::fat::FAT;
use crate::fs::vbr::VBR;
use crate::sector::Sector;

#[cfg(test)]
mod tests;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq)]
pub struct Partition {
    pub(crate) offset: u16,
    pub(crate) flag_byte: u8,
    pub(crate) first_lba: u32,
    pub(crate) first_sector: CHS,
    pub(crate) partition_type: u8,
    pub(crate) last_sector: CHS,
    pub(crate) sector_count: u32,
    pub(crate) last_lba: u32,
    sectors: Vec<Sector>, // Sectors shouldn't be accessible from anywhere else
}

impl Partition {
    /// Instantiate a new Partition struct on a specific disk.
    pub fn new(
        disk: &Disk,
        partition_number: u8,
        mut start_sector: u32,
        partition_bytes: u64,
    ) -> Partition {
        let sector_size: usize = 512;

        // Can't have things begin before sector 63. Theoretically, sure, but MS-DOS doesn't do it that way.
        if start_sector < 63 {
            start_sector = 63;
        }

        let mut requested_sectors: u32 =
            u32::try_from(partition_bytes).unwrap() / u32::try_from(sector_size).unwrap();

        let last_chs = CHS::new(&disk.geometry.cylinder - 1, disk.geometry.head - 1, 63);

        let last_lba = disk.chs_to_lba(&last_chs);

        if partition_bytes == 0 {
            requested_sectors = last_lba - 62;
        }

        // MBR layout doesn't support more than 4 primary partitions. Extended partitioning is out of scope (for now).
        if partition_number > 4 || partition_number == 0 {
            panic!("Can't have more than 4 partitions, starting at offset 1. You tried to create one at the wrong place.");
        }

        // Compose the Partition struct and return it.
        let my_partition = Partition {
            offset: 0x1be,
            flag_byte: 0x80,
            first_sector: CHS::from_lba(&disk.geometry, start_sector),
            partition_type: 0x06,
            last_sector: last_chs,
            first_lba: start_sector,
            last_lba: last_lba,
            sector_count: requested_sectors,
            sectors: Vec::<Sector>::new(),
        };
        return my_partition;
    }

    /// Format the partition as FAT16
    pub fn format(&mut self) {
        let fat = FAT::new(self.sector_count);
        let vbr = VBR::new(self.sector_count);
        let mut bootsector = Sector::new(usize::try_from(self.first_lba).expect("Failed conversion!"));
        for (i, byte) in vbr.as_bytes().iter().enumerate() {
            bootsector.write_byte(i, *byte);
        }
        self.sectors.push(bootsector);
    }

    /// Return the Sectors as an immutable ref, prevent modification
    /// from outside the partition.
    pub fn get_sectors(&self) -> &Vec::<Sector> {
        &self.sectors
    }

    /// The first byte of the partition on the underlying disk, as a u64 for easy consumption by StreamSlice
    pub fn get_start_offset(&self) -> u64 {
        let start_offset = self.first_lba * 512;
        return u64::from(start_offset);
    }
    /// The last byte of the partition on the underlying disk, as a u64 for easy consumption by StreamSlice
    pub fn get_end_offset(&self) -> u64 {
        let end_offset = self.last_lba * 512;
        return u64::from(end_offset);
    }

    /// Return the bytes to be written to the MBR's partition table.
    pub fn mbr_entry(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        let start_chs = self.first_sector.as_bytes();
        let end_chs = self.last_sector.as_bytes();
        bytes.push(self.flag_byte);
        for byte in start_chs {
            bytes.push(byte);
        }
        bytes.push(self.partition_type);
        for byte in end_chs {
            bytes.push(byte);
        }
        for byte in self.first_lba.to_le_bytes() {
            bytes.push(byte);
        }
        for byte in self.sector_count.to_le_bytes() {
            bytes.push(byte);
        }
        return bytes;
    }

    /// Generate a Partition struct from an MBR entry
    pub fn from_bytes(entry: [u8; 16]) -> Partition {
        let sector_count = u32::from_le_bytes(
            entry[12..16]
                .try_into()
                .expect("Failed to convert bytes to sector count."),
        );

        let mut first_chs_bytes = [0u8; 3];
        first_chs_bytes[0] = entry[1];
        first_chs_bytes[1] = entry[2];
        first_chs_bytes[2] = entry[3];

        let mut last_chs_bytes = [0u8; 3];
        last_chs_bytes[0] = entry[5];
        last_chs_bytes[1] = entry[6];
        last_chs_bytes[2] = entry[7];

        let first_lba = u32::from_le_bytes(
            entry[8..12]
                .try_into()
                .expect("Failed to convert bytes into LBA."),
        );

        Partition {
            offset: 0x1be,
            flag_byte: entry[0],
            last_sector: CHS::from_bytes(last_chs_bytes),
            first_sector: CHS::from_bytes(first_chs_bytes),
            partition_type: 0x06,
            first_lba: first_lba,
            sector_count: sector_count,
            last_lba: sector_count + (first_lba - 1),
            sectors: Vec::<Sector>::new(),
        }
    }
}
