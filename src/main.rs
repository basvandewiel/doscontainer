use clap::Parser;
use doscontainer::disk::Disk;
use doscontainer::partition::Partition;

#[derive(Parser, Debug)]
#[clap(version, about = "DOS Container generates MS-DOS compatible disk images.", long_about = None)]
struct Args {
    /// Path to the disk image to create
    #[clap(short, long)]
    path: String,

    /// Disk size in bytes
    #[clap(short, long)]
    size: usize,

    /// Debug flag
    #[clap(short, long)]
    debug: bool,
}
fn main() {
    let args = Args::parse();
    let mut disk = Disk::new(args.path.as_str(), args.size);
    let bootpart = Partition::new(&disk, 1, 6, 0);
    if args.debug {
        println!("{:?}", bootpart);
    }
    disk.push_partition(bootpart);
    disk.write();
    // let disk = Disk::load(&args.path);
    // println!("{:?}", disk);
}
