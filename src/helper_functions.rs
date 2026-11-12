use crate::io::SeekFrom;
use crate::command::Command;
use std::fs::{self, File};
use std::io;
use std::io::Seek;
use std::io::Read;
use crate::db::Db;

pub fn get_bytes(file: &mut File,start: u64, amount: u64) -> Vec<u8>{
    let _ = file.seek(SeekFrom::Start(start));
    let mut buffer = vec![0u8;amount as usize];
    let _ = file.read_exact(&mut buffer);
    let _ = file.seek(SeekFrom::Start(0));
    buffer
}

pub fn get_length(db: &mut Db) -> io::Result<u64>{
    let metadata = fs::metadata(db.path.clone())?;
    let file_size: u64 = metadata.len();
    let counter = db.content_length as u64 + 4*db.dimensions_amount;
    let result = (file_size - 9) / counter;
    return Ok(result);
}

pub fn get_vector_at_index(i: u64, db: &mut Db) -> Vec<f32> {
    let content_len = db.content_length as u64;
    let dims = db.dimensions_amount as u64;
    let mut vector = Vec::new();

    for j in 0..db.dimensions_amount {
        let iterator = 9 + i * (content_len + dims * 4) + content_len + (j as u64) * 4;
        
        let bytes = get_bytes(&mut db.file.as_mut().unwrap(), iterator, 4);
        let bytes_array: [u8; 4] = bytes.try_into().expect("Expected 4 bytes for f32");
        let num = f32::from_le_bytes(bytes_array);
        
        vector.push(num);
    }

    vector
}

pub fn get_two_vectors(command: &Command, db: &mut Db) -> Result<Vec<Vec<f32>>, io::Error> {
    let validate_index = |val: f32| -> Result<u64, io::Error> {
        if val < 0.0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Index out of bound"),
            ));
        }
        if val as i64 as f32 != val {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Index cannot be float"),
            ));
        }
        Ok(val as u64)
    };

    // --- Обработка первого индекса ---
    let i = validate_index(command.sarg[0])?;
    let vector1 = get_vector_at_index(i, db);

    let mut vector2 = Vec::new();

    // --- Обработка второго индекса (если предоставлен) ---
    if command.sarg.len() > 1 && command.sarg[1] >= 0.0 {
        let i2 = validate_index(command.sarg[1])?;
        vector2 = get_vector_at_index(i2, db);
    } else {
        // Если передан только 1 индекс, заполняем второй вектор нулями
        vector2 = vec![0.0; db.dimensions_amount as usize];
    }

    // Возвращаем оба вектора, обернутые в Ok
    Ok(vec![vector1, vector2])
}

pub fn cosine_math(vector1:&Vec<f32>,vector2:&Vec<f32>) -> f32{
    // Calculate dot product and magnitudes for cosine similarity
    let mut dot_product = 0.0;
    let mut norm1: f32 = 0.0;
    let mut norm2: f32 = 0.0;

    for i in 0..vector1.len() {
        dot_product += vector1[i] * vector2[i];
        norm1 += vector1[i].powi(2);
        norm2 += vector2[i].powi(2);
    }

    // Prevent division by zero if any vector contains only zeros
    let similarity = if norm1 == 0.0 || norm2 == 0.0 {
        0.0
    } else {
        dot_product / (norm1.sqrt() * norm2.sqrt())
    };

    similarity
}

pub fn dot_math(vector1:&Vec<f32>,vector2:&Vec<f32>) -> f32{
    let mut result:f32 = 0.0;

    for i in 0..vector1.len(){
        result += vector1[i] * vector2[i];
    }
    result
}

pub fn euclid_math(vector1:&Vec<f32>,vector2:&Vec<f32>) -> f32{
    let mut result:f32 = 0.0;

    for i in 0..vector1.len(){
        result +=(vector1[i] - vector2[i]).powi(2);
    }
    result.sqrt()
}

pub fn manhattan_math(vector1:&Vec<f32>,vector2:&Vec<f32>) -> f32{
    let mut result:f32 = 0.0;

    for i in 0..vector1.len(){
        result = result + (vector1[i] - vector2[i]).abs();
    }
    result
}