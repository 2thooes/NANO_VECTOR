use crate::io::SeekFrom;
use crate::command::Command;
use std::fs::{self, File};
use std::io;
use std::io::Seek;
use std::io::Read;
use crate::db::Db;

pub fn get_bytes(file: &mut File,start: u64, amount: u64) -> Vec<u8>{
    file.seek(SeekFrom::Start(start));
    let mut buffer = vec![0u8;amount as usize];
    file.read_exact(&mut buffer);
    file.seek(SeekFrom::Start(0));
    buffer
}

pub fn get_vector_element(command: &Command, db: &mut Db) -> Result<Vec<Vec<f32>>, io::Error> {
    // Check if the first index is out of bounds (negative)
    if command.sarg[0] < 0.0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Index out of bound"),
        ));
    }
    //Error handling in case if index is float
    if command.sarg[0] as i64 as f32 != command.sarg[0] {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Index cannot be float"),
        ));
    }

    let mut vector1 = Vec::new();
    let mut vector2 = Vec::new();

    // Read metadata of the database file
    let metadata = match fs::metadata(&db.path) {
        Ok(meta) => meta,
        Err(_err) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Failed to read metadata"),
            ))
        }
    };
    
    let file_size: u64 = metadata.len(); // Amount of bytes in file
    
    let content_len = db.content_length as u64;
    let dims = db.dimensions_amount as u64;
    
    // --- Processing the first index ---
    let i = command.sarg[0] as u64;

    // Reading coordinates after the data for vector 1
    for j in 0..db.dimensions_amount {
        let iterator = 9 + i * (content_len + dims * 4) + content_len + (j as u64) * 4;
        
        let bytes = get_bytes(&mut db.file.as_mut().unwrap(), iterator, 4);
        let bytes_array: [u8; 4] = bytes.try_into().expect("Expected 4 bytes for f32");
        let num = f32::from_le_bytes(bytes_array);
        
        vector1.push(num);
    }

    // --- Processing the second index (if provided) ---
    // Check if the command contains a second argument/index
    if command.sarg.len() > 1 && command.sarg[1] >= 0.0 {
        //Error handling in case if index is float
        if command.sarg[1] as i64 as f32 != command.sarg[1] {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Index cannot be float"),
            ));
        }

        let i2 = command.sarg[1] as u64;
        
        // Reading coordinates for the second vector using the second index
        for j in 0..db.dimensions_amount {
            let iterator = 9 + i2 * (content_len + dims * 4) + content_len + (j as u64) * 4;
            
            let bytes = get_bytes(&mut db.file.as_mut().unwrap(), iterator, 4);
            let bytes_array: [u8; 4] = bytes.try_into().expect("Expected 4 bytes for f32");
            let num = f32::from_le_bytes(bytes_array);
            
            vector2.push(num);
        }
    } else {
        // If only 1 index is passed, fill the second vector with zeros matching dimensions amount
        vector2 = vec![0.0; db.dimensions_amount as usize];
    }

    // Return both vectors wrapped in Ok
    Ok(vec![vector1, vector2])
}