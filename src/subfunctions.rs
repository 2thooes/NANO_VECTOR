use crate::helper_functions;
use crate::command::Command;
use crate::db::Db;
use std::fs;
use std::io;

pub fn get_all(command: &Command, db: &mut Db) -> io::Result<String> {
    let metadata = match fs::metadata(&db.path){
        Ok(meta) => meta,
        Err(err) => {
            return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read metadata"),
        ))
        }
    };
    let file_size: u64 = metadata.len();//abount of bytes in file
    
    let content_len = db.content_length as u64;
    let dims = db.dimensions_amount as u64;
    let step = content_len + dims * 4;

    let mut i = 0;
    let mut current_pos = 9;
    //starting from 9, to the end through each data slice
    while current_pos + step <= file_size {
        // Reading content
        let text_bytes = helper_functions::get_bytes(&mut db.file.as_mut().unwrap(), current_pos, content_len);
        // Something
        let text_end = text_bytes.iter().position(|&b| b == 0).unwrap_or(text_bytes.len());
        let text = String::from_utf8_lossy(&text_bytes[..text_end]);
        print!("{} ", text);

        // Reading coordinates after the data
        let vector_start = current_pos + content_len;
        print!("<");
        for j in 0..db.dimensions_amount {
            let iterator = vector_start + (j as u64) * 4;
            
            let bytes = helper_functions::get_bytes(&mut db.file.as_mut().unwrap(), iterator, 4);
            let bytes_array: [u8; 4] = bytes.try_into().expect("Expected 4 bytes for f32");
            let num = f32::from_le_bytes(bytes_array);
            
            print!("<{}>", num);
        }
        println!(">");
        current_pos += step;
        i += 1;
    }

    Ok("Success!".to_string())
}

pub fn get_id(command: &Command, db: &mut Db) -> io::Result<String>{
    if command.sarg.is_empty(){
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Not enough arguments"),
        ))
    }
    if command.sarg.len() > 1{
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Too much arguments"),
        ))
    }
    let metadata = match fs::metadata(&db.path){
        Ok(meta) => meta,
        Err(err) => {
            return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read metadata"),
        ))
        }
    };
    let i = command.sarg[0] as u64;
    if i as f32 != command.sarg[0]{
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Cannot use float id"),
        ))
    }
    let file_size: u64 = metadata.len();//amount of bytes in file
    
    let content_len = db.content_length as u64;
    let dims = db.dimensions_amount as u64;
    let i = command.sarg[0] as u64;
    // Reading content
    let text_bytes = helper_functions::get_bytes(&mut db.file.as_mut().unwrap(), 9 + i*(content_len + dims * 4), content_len);
    // Something
    let text_end = text_bytes.iter().position(|&b| b == 0).unwrap_or(text_bytes.len());
    let text = String::from_utf8_lossy(&text_bytes[..text_end]);
    print!("{} ", text);
    // Reading coordinates after the data
    print!("<");
    for j in 0..db.dimensions_amount {
        let iterator = 9 + i*(content_len + dims * 4) + content_len + (j as u64) * 4;
        
        let bytes = helper_functions::get_bytes(&mut db.file.as_mut().unwrap(), iterator, 4);
        let bytes_array: [u8; 4] = bytes.try_into().expect("Expected 4 bytes for f32");
        let num = f32::from_le_bytes(bytes_array);
            
        print!("<{}>", num);
    }
    println!(">");

    Ok("Success!".to_string())
}

pub fn search_cosine_byid(command: &Command, db: &mut Db) -> io::Result<String>{
    if command.sarg.len() < 1{
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Not enough arguments")
        ))
    }
    if command.sarg.len() > 2{
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Too many arguments")
        ))
    }
    if command.sarg.len() == 1{

    }
    return Ok("Euclid was called".to_string());
}

pub fn search_dot_byid(command: &Command, db: &mut Db) -> io::Result<String>{
    return Ok("Dot was called".to_string());
}

pub fn search_euclid_byid(command: &Command, db: &mut Db) -> io::Result<String>{
    return Ok("Euclid was called".to_string());
}

pub fn search_manhattan_byid(command: &Command, db: &mut Db) -> io::Result<String>{
    if command.sarg.len() < 1{
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Not enough arguments")
        ))
    }
    if command.sarg.len() > 2{
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Too many arguments")
        ))
    }
    if command.sarg.len() == 2{
        let result = 0;//дописать
    }
    return Ok("Euclid was called".to_string());
}