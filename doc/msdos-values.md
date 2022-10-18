# MS-DOS test case

## 50MB disk file
Steps to reproduce:

1. Generate a raw disk image of exactly 49999872 bytes in size.
2. Install MS-DOS 6.22 to it, shut down your machine after the C:> prompt appears.
3. Use the Disk::load() function to read the resulting image.

The resulting values for the root partition are:

```
geometry: CHS { cylinder: 96, head: 16, sector: 63 }, 

Partition
  offset: 446, 
  flag_byte: 128, 
  first_lba: 63, 
  first_sector: CHS { cylinder: 0, head: 1, sector: 1 }, 
  partition_type: 6, 
  last_sector: CHS { cylinder: 95, head: 15, sector: 15 }, 
  sector_count: 96705,
```

The actual on-disk partition table entry must be:

```
80 01 01 00 06 0F 3F 5F 3F 00 00 00 C1 79 01 00
```

## 100MB disk file

Same steps to reproduce, but the image file must be 99999744 in size.

```
geometry: CHS { cylinder: 193, head: 16, sector: 63 }, 
Partition
  offset: 446, 
  flag_byte: 128, 
  first_lba: 63, 
  first_sector: CHS { cylinder: 0, head: 1, sector: 1 }, 
  partition_type: 6, 
  last_sector: CHS { cylinder: 192, head: 15, sector: 63 },
  sector_count: 194481,
  last_lba: 194418
```

The on-disk partition table for a 100MB disk file:

```
80 01 01 00 06 0F 3F C0 3F 00 00 00 B1 F7 02 00
```
