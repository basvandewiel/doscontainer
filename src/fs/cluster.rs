use crate::sector::Sector;

#[derive(Debug, PartialEq)]
pub struct Cluster {
    value: u16,
    sectors: Vec<Sector>,
}

impl Cluster {
    pub fn new(value: u16) -> Self {
        Cluster {
            value: value,
            sectors: Vec::<Sector>::new(),
        }
    }

    pub fn get_value(&self) -> u16 {
        self.value
    }
}
