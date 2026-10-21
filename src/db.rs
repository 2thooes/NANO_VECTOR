use std::path::PathBuf;
use std::fs::File;

#[derive(Default)]
pub struct Db{
    pub path:PathBuf,
    pub file:Option<File>,
    pub dimensions_amount: u64,
    pub content_length:u8,
}

impl Db {
    // Creating empty objects
    pub fn null() -> Self {
        Db {
            path: PathBuf::new(), // Empty path
            dimensions_amount: 0,
            content_length:0,
            file:None,
        }
    }
}

impl PartialEq for Db{
    fn eq(&self,other: &Self) -> bool{
        self.path == other.path && 
        self.dimensions_amount == other.dimensions_amount && 
        self.content_length == other.content_length 
    }
}

impl Eq for Db {}