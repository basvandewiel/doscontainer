pub mod chs;
pub mod disk;

/// The FS module is meant to supplement what the fatfs crate already gives us.
/// It mainly implements the VBR, BIOS Parameter block and the MS-DOS 6.22 Volume Boot Code.
pub mod fs;
pub mod partition;
mod tests;