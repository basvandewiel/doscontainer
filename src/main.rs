use doscontainer::Disk;
use doscontainer::Partition;
use doscontainer::CHS;
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
    // my_disk.write();
    let start_sector = my_disk.lba_to_chs(63);
    my_disk.partitions.push(Partition::new(1, start_sector, args.size));
    my_disk.write();
    // println!("Create file at: {}", args.path);
    // println!("Disk size will be: {} byes.", args.size);
    println!("{:?}", my_disk);
}
