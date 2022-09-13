use doscontainer::Disk;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about = "DOS Container generates MS-DOS compatible disk images.", long_about = None)]
struct Args {
    /// Path to the disk image to create
    #[clap(short, long)]
    path: String,

    /// Disk size in bytes
    #[clap(short, long)]
    size: u32,
}
fn main() {
    let args = Args::parse();
    let mut my_disk = Disk::new(args.path.as_str(), args.size);
    my_disk.write();
    println!("Create file at: {}", args.path);
    println!("Disk size will be: {} byes.", args.size);
}
