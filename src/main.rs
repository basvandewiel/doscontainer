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
    size: u64,
}
fn main() {
    let args = Args::parse();
    let my_disk = Disk::load(args.path.as_str());
    println!("{:?}", my_disk);
}
