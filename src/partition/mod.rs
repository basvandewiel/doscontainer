use crate::chs::*;
use crate::disk::*;
use crate::fs::vbr::VBR;
use crate::fs::FAT;

/// Custom type for a Partition
#[derive(Debug)]
pub struct Partition {
    pub(crate) offset: u16,
    pub(crate) flag_byte: u8,
    pub(crate) first_lba: u32,
    pub(crate) first_sector: CHS,
    pub(crate) partition_type: u8,
    pub(crate) last_sector: CHS,
    pub(crate) sector_count: u32,
    pub(crate) last_lba: u32,
    pub(crate) boot_record: VBR,
    pub(crate) FAT: FAT,
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
        let mut requested_sectors: u32 =
            u32::try_from(partition_bytes).unwrap() / u32::try_from(sector_size).unwrap();

        // Special case: 0 grows the partition to fill the entire disk
        if requested_sectors == 0 {
            let disk_sectors = disk.size / sector_size;
            let free_sectors = disk_sectors - 64;
            requested_sectors = u32::try_from(free_sectors).unwrap();
        }

        // MBR layout doesn't support more than 4 primary partitions. Extended partitioning is out of scope (for now).
        if partition_number > 4 || partition_number == 0 {
            panic!("Can't have more than 4 partitions, starting at offset 1. You tried to create one at offset {}", partition_number);
        }

        // Can't have things begin before sector 63. Theoretically, sure, but MS-DOS doesn't do it that way.
        if start_sector < 63 {
            start_sector = 63;
        }

        // Cylinder-align the partition's end. Try to get as close to FDISK.EXE as we can.
        // Update requested_sectors with the aligned value fom the CHS.
        let mut end_chs = disk.lba_to_chs(requested_sectors);
        end_chs.head = 15;
        end_chs.sector = 63;
        end_chs.cylinder -= 2;
        requested_sectors = disk.chs_to_lba(&end_chs);

        // Compose the Partition struct and return it.
        let my_partition = Partition {
            offset: 0x1be,
            flag_byte: 0x80,
            first_sector: disk.lba_to_chs(start_sector),
            partition_type: 0x06,
            last_sector: end_chs,
            first_lba: start_sector,
            last_lba: requested_sectors - start_sector,
            sector_count: requested_sectors,
            boot_record: VBR::new(requested_sectors),
            FAT: FAT::new(requested_sectors),
        };

        return my_partition;
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
    pub fn as_bytes(&self) -> Vec<u8> {
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
        Partition {
            offset: 0x1be,
            flag_byte: entry[0],
            partition_type: 0x06,
            first_lba: entry[8],
            boot_record: VBR::new(25),
        }
    }
}
