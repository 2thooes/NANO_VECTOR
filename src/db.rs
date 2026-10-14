use std::io;
use std::path::PathBuf;

#[derive(Default)]
pub struct Db{
    pub path:PathBuf,
    pub dimensions_amount: u64,
}

impl Db {
    // Creating empty objects
    pub fn null() -> Self {
        Db {
            path: PathBuf::new(), // Empty path
            dimensions_amount: 0,
        }
    }
}

impl PartialEq for Db{
    fn eq(&self,other: &Self) -> bool{
        self.path == other.path && self.dimensions_amount == other.dimensions_amount
    }
}

impl Eq for Db {}