use std::path::PathBuf;

#[derive(Debug)]
pub struct Disk {
    path: PathBuf,
}

impl Disk {
    pub fn new(path: &str, size: u64) -> Disk {
        Disk {
            path: PathBuf::from(path),
        }
    }
}
