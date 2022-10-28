# Partitions

A partition in the context of MS-DOS is a section of a fixed disk that contains a filesystem and, by extension, the user's actual data. Ignoring extended partitions for now, MS-DOS supports a maximum of 4 partitions per drive. The use case of Doscontainer is such that it's highly unlikely to ever need more than 4 partitions, more than 1 is stretching it initially. Roadmap-wise Doscontainer will therefore support 1 partition initially, up to 4 eventually and extended partitions only if a contributor brings in the code for it.

## Architecture

In theory a partition addresses two separate concerns:

- One of the 4 entries in the Disk's boot record partition table.
- The on-disk partition itself that contains the filesystem.

The partition table entry is a sequence of 16 bytes that represents the dimensions of the partition on the disk. The implementation varied slightly over the years during the time period Doscontainer intends to cover. Because modern storage is cheap, we don't really need to bother with the really ancient use cases initially. This is why I chose to build Doscontainer to mimic MS-DOS 6.22 as closely as possible, it being the last independent mainstream MS-DOS release before DOS got merged into Windows 9x and eventually disappeared altogether. In practice this means that all unit tests are built to reflect exactly what MS-DOS 6.22 would do in their case.

## Why we ignore the on-disk partition

The interesting bit about these old-school partitions is that nothing actually happens outside the MBR partition table when manipulating them. The area of the disk that they allocate does not get touched when a partition is created. The only artifact that's visible is a change to the table entry. It is only by convention that the MS-DOS filesystem driver and utilities like FORMAT respect the boundaries of a partition on the rest of the physical medium.

We work directly with the Disk struct, as the owner of the pool of Sectors, for all on-disk data manipulation. All we need to do is colour within the lines of the first/last sector of the partition as defined in the MBR.
