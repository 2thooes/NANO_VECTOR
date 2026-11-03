use crate::io::SeekFrom;
use crate::command::Command;
use std::fs::{self, File};
use std::io::{Read,Seek};

pub fn get_bytes(file: &mut File,start: u64, amount: u64) -> Vec<u8>{
    file.seek(SeekFrom::Start(start));
    let mut buffer = vec![0u8;amount as usize];
    file.read_exact(&mut buffer);
    file.seek(SeekFrom::Start(0));
    buffer
}