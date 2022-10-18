/// Data structure for individual sectors. A sector holds 512 bytes of data and is
/// the smallest unit of data a Disk can work with. The data is kept in a Vec<u8> internally.
/// The position of the sector is the LBA address and we keep a 'dirty' flag to see if the
/// sector is present on the disk.
#[derive(Debug, PartialEq)]
pub struct Sector {
    data: Vec<u8>,
    dirty: bool,
    position: usize,
}

impl Sector {
    /// Create a new Sector
    pub fn new(position: usize) -> Self {
        Sector {
            data: vec![0; 512],
            dirty: true,
            position: position,
        }
    }

    /// Returns the position of the sector on a Disk
    pub fn get_position(&self) -> usize {
        self.position
    }

    /// Set the position of the Sector on a Disk
    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    /// Marks the Sector as clean
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Marks the Sector as dirty
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Returns true if the sector is dirty, false if it's not.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Write a byte to a position inside the sector. Use a function
    /// for this to gatekeep the 'dirty' flag.
    pub fn write_byte(&mut self, position: usize, value: u8) {
        if position > self.data.len() {
            panic!("Position out of bounds while writing to sector.");
        }
        self.data[position] = value;
        self.mark_dirty();
    }

    pub fn get_data(&self) -> [u8; 512] {
        let mut val = [0u8; 512];
        for (index, byte) in self.data.iter().enumerate() {
            val[index] = *byte;
        }
        val
    }
}
