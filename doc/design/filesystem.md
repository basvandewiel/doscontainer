# Filesystem

The filesystem of choice for Doscontainer is FAT16. In ancient use cases and on tiny hard drives FAT12 used to be a factor. I'm choosing to ignore that for now because I don't want to deal with the fact that FAT12 stores its records as 12-bit values, and we need to bit-twiddle those across 8-bit bytes all for a marginal return of being able to faithfully reproduce tiny hard drives that won't be functionally impaired if formatted as FAT16. Eventually, when Doscontainer reaches a viable 1.0 version, I'll revisit FAT12 and probably also older MBR formats. For now: FAT16 is the only supported filesystem.

## Architecture

A FAT16 file system globally consists of three items:

- The Volume Boot Record
- The FAT's themselves
- The on-disk data area where actual files get stored.

## Volume Boot Record

The VBR spells out the rest of the filesystem's on-disk layout, number of FAT's, their size, numbers of sectors per cluster etc. It also provides the second stage of code for the computer to load an operating system. It follows a strict design structure and is represented in Doscontainer by the Vbr struct.

## File Allocation Table

The FAT struct encapsulates the File Allocation Table for Doscontainer. It allows a consumer to allocate clusters, which in turn translate to sectors on the Disk struct in accordance with the number of sectors per cluster in the VBR.

## On-disk data area

The data area is a flat sequence of Sectors. The Sectors are grouped into Clusters by FAT, but that distinction does not carry over onto the on-disk structure other than there being gaps between the data of files that coincide with cluster boundaries. So effectively a file that's bound for the disk will be chopped into cluster-sized chunks, the FAT will be asked to provide sufficient free clusters, and the actual data itself will be sequentially written across the sectors that are covered by the Clusters. When we run out of data before the end of the cluster, the bytes will be zero.
