# Disk struct

The purpose of the Disk struct is to function as the intermediary between the
image file that lives on the host's persistent storage, and the higher-level
structures that MS-DOS requires in the emulated environment.

From the MS-DOS end of things, a Disk is a pool of Sectors that overlying
structures can consume. The Disk ensures that those Sectors exist, and are
properly persisted to the host system when work is done.

## Physical characteristics

Fixed disks in the MS-DOS age used to have a strictly defined physical geometry
measured in Cylinders, Heads and Sectors. As the geometry is a property of the
whole disk, it's also a property of our Disk struct.

Virtual disk image files obviously have no relation to the physical characteristics
of real fixed disk drives. In order to properly emulate them, therefore, we should
look at how the emulator represents them.

Doscontainer is built primarily for MiSTer's AO486 core, which is a reproduction
of the Bochs emulator in FPGA. Sure I'm cutting corners here, but looking at it
from the perspective of the disk subsystem it's a clone. The critical part of
this being the BIOS and how it represents a virtualized disk's geometry to the
emulated operating system.

The Bochs BIOS implements a number of algorithms to convert a given disk size
in bytes to a CHS-geometry. The Disk struct supports only a single one at present,
but preparations are there to support others as needed. For now, the single implemented
algorithm is the 'none' case, where no BIOS-aided translation is needed. The main
disadvantage of this is that disk sizes above 528MB are unavailable.

The implementation works in a staggered way. As a consumer, you call ```calculate_geometry(size: usize)```
and from there a private implementation is chosen depending on the size of the disk
you want to work with. This method hands back a CHS-tuple that describes the geometry.

## Logical structures

Some structures on a fixed disk are particular to the disk itself. That is why they
are implemented in the Disk struct. Specifically the boot sector is a construct that
makes sense only in the context of a whole disk. In order to construct a proper boot
sector, the Disk struct needs information about operating system boot code (a bit of
x86 assembly code) and the partition table.

## Sectors

Real fixed disks used to exist with 4096-byte sectors, but I have yet to encounter a
single emulator that presents such a disk to the guest OS. That's why I'm working 
with a hard-coded sector size of 512 bytes. When allocating a disk, you provide the
size you want and the Disk struct ensures that you'll get one that aligns on the first
512-byte boundary found below the size you requested.

Consumers of the Disk struct can use its methods to pull out a section of the Disk's
Sectors. The word Sector here is capitalized because it's a separate Rust struct with
its own methods and metadata. The Disk struct takes care of the logistics of reading
and writing them to the host file. Nowhere in Doscontainer does anything get written to
or read from the host filesystem other than in the Disk struct.
