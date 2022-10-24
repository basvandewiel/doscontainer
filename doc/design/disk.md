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

Doscontainer is built primarily for MiSTEr's AO486 core, which is a reproduction
of the Bochs emulator in FPGA. Sure I'm cutting corners here, but looking at it
from the perspective of the disk subsystem it's a clone. The critical part of
this being the BIOS and how it represents a virtualized disk's geometry to the
emulated operating system.
