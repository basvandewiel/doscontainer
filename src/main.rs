use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the disk image to create
    #[clap(short, long)]
    path: String,
}
fn main() {
    let args = Args::parse();
    println!("Create file at: {}", args.path);
}
