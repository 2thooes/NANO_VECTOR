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
            format!("{}",err),
        ))
        }
    };
    let file_size: u64 = metadata.len();//abount of bytes in file
    
    let content_len = db.content_length as u64;
    let dims = db.dimensions_amount as u64;
    let step = content_len + dims * 4;


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

pub fn closest(command: &Command, db: &mut Db) -> io::Result<String>{
    if command.sarg.is_empty(){
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Not enough arguments"),
        ))
    }
    if command.sarg.len() > 2{
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Too many arguments"),
        ))
    } // continue
    Ok("s".to_string())
}

pub fn cosine(command: &Command, db: &mut Db) -> io::Result<f32> {
    // Call get_vector_element and handle potential I/O or index errors using '?'
    let result = helper_functions::get_two_vectors(command, db)?;
    let mut vectors: Vec<Vec<f32>>;
    let vectors = match helper_functions::get_two_vectors(command, db) {
        Ok(vec) => vec,
        Err(e) => {
            return Err(e);
        }
    };
    // Ensure we actually have two vectors to compare
    if vectors.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Expected two vectors for cosine calculation"),
        ));
    }

    let vector1 = &vectors[0];
    let vector2 = &vectors[1];

    let result:f32 = helper_functions::cosine_math(vector1,vector2);

    // Return the calculated similarity result as a string
    Ok(result)
}

pub fn dot(command: &Command, db:&mut Db) -> io::Result<f32>{
    // Call get_vector_element and handle potential I/O or index errors using '?'
    let result = helper_functions::get_two_vectors(command, db)?;
    let mut vectors: Vec<Vec<f32>>;
    let vectors = match helper_functions::get_two_vectors(command, db) {
        Ok(vec) => vec,
        Err(e) => {
            return Err(e); 
        }
    };
    // Ensure we actually have two vectors to compare
    if vectors.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Expected two vectors for cosine calculation"),
        ));
    }

    let vector1 = &vectors[0];
    let vector2 = &vectors[1];

    let mut result:f32 = 0.0;

    for i in 0..vector1.len(){
        result += vector1[i] * vector2[i];
    }
    Ok(result)
}

pub fn euclid(command: &Command, db:&mut Db) -> io::Result<f32>{
    // Call get_vector_element and handle potential I/O or index errors using '?'
    let result = helper_functions::get_two_vectors(command, db)?;
    let mut _vectors: Vec<Vec<f32>>;
    let vectors = match helper_functions::get_two_vectors(command, db) {
        Ok(vec) => vec,
        Err(e) => {
            return Err(e); 
        }
    };
    // Ensure we actually have two vectors to compare
    if vectors.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Expected two vectors for cosine calculation"),
        ));
    }

    let vector1 = &vectors[0];
    let vector2 = &vectors[1];

    let mut result:f32 = 0.0;

    for i in 0..vector1.len(){
        result +=(vector1[i] - vector2[i]).powi(2);
    }
    Ok(result.sqrt())
}

pub fn manhattan(command: &Command, db:&mut Db) -> io::Result<f32>{
    // Call get_vector_element and handle potential I/O or index errors using '?'
    let result = helper_functions::get_two_vectors(command, db)?;
    let mut vectors: Vec<Vec<f32>>;
    let vectors = match helper_functions::get_two_vectors(command, db) {
        Ok(vec) => vec,
        Err(e) => {
            return Err(e); 
        }
    };
    // Ensure we actually have two vectors to compare
    if vectors.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Expected two vectors for cosine calculation"),
        ));
    }

    let vector1 = &vectors[0];
    let vector2 = &vectors[1];

    let mut result:f32 = 0.0;

    for i in 0..vector1.len(){
        result = result + (vector1[i] - vector2[i]).abs();
    }
    Ok(result)
}

pub fn closest_by_cosine(command: &Command, db:&mut Db) -> io::Result<String>{
    let num = command.sarg[0] as usize;
    let vector:Vec<f32> = helper_functions::get_vector_at_index(command.sarg[1] as u64,db);
    let mut result: Vec<(u64,f32)> = vec!();
    
    for i in 0..helper_functions::get_length(db).unwrap(){
        if i != command.sarg[1] as u64{
            let res = helper_functions::cosine_math(&helper_functions::get_vector_at_index(i as u64,db),&vector);
            result.push((i,res));
        }
        
    }
    result.sort_unstable_by(|a, b| {
        b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
    });
    for i in 0..num{
        let (idx, score) = result[i];
        println!("{},{}",idx, score);
    }
    Ok(format!("Success!"))
}

pub fn closest_by_dot(command: &Command, db:&mut Db) -> io::Result<String>{
    Ok(format!("Ok"))
}

pub fn closest_by_euclid(command: &Command, db:&mut Db) -> io::Result<String>{
    Ok(format!("Ok"))
}

pub fn closest_by_manhattan(command: &Command, db:&mut Db) -> io::Result<String>{
    Ok(format!("Ok"))
}