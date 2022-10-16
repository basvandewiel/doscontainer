/// The disk module handles the virtual disk image file itself.
pub mod disk;

/// The sector module separates out all code that deals with handling sectors on disks.
pub mod sector;

/// The FS module is meant to supplement what the fatfs crate already gives us.
/// It mainly implements the VBR, BIOS Parameter block and the MS-DOS 6.22 Volume Boot Code.
pub mod fs;
pub mod partition;
