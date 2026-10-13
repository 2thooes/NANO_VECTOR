use std::io;
use std::path::PathBuf;

pub struct Db{
    pub path:PathBuf,
    pub dimentions_amount: u64,
}