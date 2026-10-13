use std::io;
use std::path::PathBuf;

#[derive(Default)]
pub struct Db{
    pub path:PathBuf,
    pub dimentions_amount: u64,
}

impl Db {
    // Creating empty objects
    pub fn null() -> Self {
        Db {
            path: PathBuf::new(), // Empty path
            dimentions_amount: 0,
        }
    }
}