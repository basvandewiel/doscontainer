use doscontainer::Disk;
use doscontainer::Partition;
use clap::Parser;

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
    my_disk.partitions.push(Partition::new(&my_disk, 1, 63, 49000000));
    my_disk.write();
    println!("{:?}", my_disk);
}
