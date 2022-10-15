use bitvec::prelude::*;

/// Custom type for Cylinder/Head/Sector geometry
#[derive(Debug, PartialEq)]
pub struct CHS {
    pub(crate) cylinder: u16,
    pub(crate) head: u8,
    pub(crate) sector: u8,
}

impl CHS {
    /// Instantiate an empty CHS tuple
    pub fn empty() -> CHS {
        CHS {
            cylinder: 0,
            head: 0,
            sector: 0,
        }
    }
    /// Calculate triplet of CHS bytes for use in partition tables
    /// <https://thestarman.pcministry.com/asm/mbr/PartTables.htm#mbr>
    pub fn as_bytes(&self) -> [u8; 3] {
        // Turn the cylinders u16 into a BitVec so we can twiddle bits
        let cylinders_as_bits = BitVec::<_, Msb0>::from_element(self.cylinder);
        let cylinders_clipped = &cylinders_as_bits[6..=15];

        // Turn the sectors u8 into a BitVec so we can twiddle bits
        let sectors_as_bits = BitVec::<_, Msb0>::from_element(self.sector);
        let sectors_clipped = &sectors_as_bits[2..=7];

        let heads_as_bits = BitVec::<_, Msb0>::from_element(self.head);

        // Two variables hold the two distinct parts of the cylinders field
        let cylinders_overflow_bits = &cylinders_clipped[0..=1];
        let cylinders_byte = &cylinders_clipped[2..=9];

        // Create a sequence of 24 bits to gather all the bits in the right sequence
        let mut collected_bits: BitVec<u32, bitvec::order::Msb0> = BitVec::new();
        collected_bits.extend_from_bitslice(&heads_as_bits);
        collected_bits.extend_from_bitslice(&cylinders_overflow_bits);
        collected_bits.extend_from_bitslice(&sectors_clipped);
        collected_bits.extend_from_bitslice(&cylinders_byte);

        // Compose the sectors field (a byte) from the cylinder overlow bits and the 6 relevant bits from sectors.
        let mut sectors_byte: bitvec::vec::BitVec<_, bitvec::order::Msb0> =
            BitVec::<u8, bitvec::order::Msb0>::new();
        sectors_byte.extend_from_bitslice(cylinders_overflow_bits);
        sectors_byte.extend_from_bitslice(sectors_clipped);

        // Convert the twiddled fields back to u8's
        let heads_as_u8 = heads_as_bits.load_le::<u8>();
        let sectors_as_u8 = sectors_byte.load_le::<u8>();
        let cylinders_as_u8 = cylinders_byte.load_le::<u8>();

        // ..and return them as an array.
        [heads_as_u8, sectors_as_u8, cylinders_as_u8]
    }
    /// Turn the encoded CHS-value from the bytes in an MBR to a CHS-tuple
    /// The order of the bytes is identical to the way they are encoded on-disk
    /// on an old MBR disk. So generally: Heads, Sectors, Cylinders in that order.
    pub fn from_bytes(bytes: [u8; 3]) -> CHS {
        // Turn the bytes into sequences of bits
        let heads_byte = BitVec::<_, Msb0>::from_element(bytes[0]);
        let sectors_byte = BitVec::<_, Msb0>::from_element(bytes[1]);
        let cylinders_byte = BitVec::<_, Msb0>::from_element(bytes[2]);

        // Put all those bits together into a Vec that's 24 bits long
        let mut chs_bits: BitVec<u8, bitvec::order::Msb0> = BitVec::new();
        chs_bits.extend_from_bitslice(&heads_byte);
        chs_bits.extend_from_bitslice(&sectors_byte);
        chs_bits.extend_from_bitslice(&cylinders_byte);

        // The heads byte comes over unmodified, it's the first 8 bits of the sequence
        let mut chs_heads: BitVec<u8, bitvec::order::Msb0> = BitVec::new();
        chs_heads.extend_from_bitslice(&chs_bits[0..=7]);

        // The sectors number is only 6 bits long, so pad it with zeroes to bring it up to 8 bits.
        let mut chs_sectors: BitVec<u8, bitvec::order::Msb0> = BitVec::new();
        chs_sectors.extend_from_bitslice(&chs_bits[9..=15]);

        // The cylinders value is 10 bits long but a u16 has room for 16, so pad with zeroes.
        let mut chs_cylinders: BitVec<u16, bitvec::order::Msb0> = BitVec::new();
        let mut i = 0;
        while i < 6 {
            chs_cylinders.push(false);
            i += 1;
        }
        // Pull in the two overflow bits that were stored in the sectors byte
        chs_cylinders.extend_from_bitslice(&chs_bits[8..=9]);
        // Finish off by adding the actual cylinders byte itself
        chs_cylinders.extend_from_bitslice(&chs_bits[16..=23]);

        // Convert all these bits back to their numerical types and create the CHS struct
        let mut chs = CHS::empty();
        chs.head = chs_heads.load_le::<u8>();
        chs.sector = chs_sectors.load_le::<u8>();
        chs.cylinder = chs_cylinders.load_le::<u16>();
        return chs;
    }
}
