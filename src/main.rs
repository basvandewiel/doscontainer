use doscontainer::Disk;
use doscontainer::Partition;
use clap::Parser;
use doscontainer::fs::fs::*;

#[derive(Parser, Debug)]
#[clap(version, about = "DOS Container generates MS-DOS compatible disk images.", long_about = None)]
struct Args {
    /// Path to the disk image to create
    #[clap(short, long)]
    path: String,

    /// Disk size in bytes
    #[clap(short, long)]
    size: u64,
}
fn main() {
    let args = Args::parse();
    let mut my_disk = Disk::new(args.path.as_str(), args.size);
    let mut part = Partition::new(&my_disk, 1, 63, 0);
    let mut vbr = VBR::new(&part);
    my_disk.partitions.push(Partition::new(&my_disk, 1, 63, 0));  
    // my_disk.write();
    println!("{:?}", my_disk);
}
