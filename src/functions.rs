use crate::command::Command;
use crate::db::Db;
use std::fs::{self, File};
use std::io;
use std::io::Read;
use std::path::PathBuf;
use directories::ProjectDirs;

// HELP command
pub fn help(command: &Command){
    if command.marg.is_empty() && command.sarg.is_empty(){
        println!("Help was called");
    }
    else{
        println!("Err:Too many arguments");
    }
}
//NEWDB command
pub fn newdb(command: &Command) -> io::Result<Db> {
    // Error handling
    if command.marg.is_empty() || command.sarg.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Err: Not enough arguments",
        ));
    }

    // 1. Searching for folder for data (for example, ~/.local/share/nano_vector in Linux)
    let proj_dirs = ProjectDirs::from("com", "2thooes", "nano_vector")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cannot identify the system folder"))?;
    
    let data_dir = proj_dirs.data_dir();

    // 2. Creating directory if possible (mkdir -p)
    fs::create_dir_all(data_dir)?;

    // 3. Generating filename using marg (for example, "my_database.bin")
    // Use format! to make .bin
    let file_name = format!("{}.bin", command.marg);
    let file_path = data_dir.join(file_name);

    // 4. Checking, if the file exists now
    if file_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Db with this name already exists",
        ));
    }

    // 5. Creating empty file
    let _file = File::create(&file_path)?;
    let header = command.sarg[0] as u64;
    fs::write(&file_path,header.to_le_bytes())?;

    // 6. Returning the address
    Ok(Db{
        path: file_path,
        dimensions_amount: header,
    })
}

pub fn status(command: &Command){
    println!("Status was called")
}

pub fn dropdb(command: &Command,db:&Db){
    println!("dropdb was called");
}

pub fn exitdb(command: &Command, db: &Db) -> io::Result<Db> {
    if !command.marg.is_empty() || !command.sarg.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "Err: Too many arguments"));
    }
    if db.dimensions_amount == 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "Err: You are not in db"));
    }
    // Returning default value
    Ok(Db::default())
}

pub fn enterdb(command: &Command,db: &Db) -> io::Result<Db> {
    //Error handlers
    if !command.sarg.is_empty(){
        return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Too many arguments"),
            ));
    }
    if command.marg.is_empty(){
        return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Not enough arguments"),
            ));
    }
    if *db != Db::default() {
        return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("You are working with other database"),
            ));
    }
    let proj_dirs = ProjectDirs::from("com", "2thooes", "nano_vector")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cannot identify the system folder"))?;

    let data_dir = proj_dirs.data_dir();
    
    // adding .bin to filename
    let file_name = format!("{}.bin", command.marg);
    let file_path = data_dir.join(file_name);

    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("{}", file_path.display());
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Файл базы данных {} не найден в папке: {:?}", command.marg, data_dir),
            ));
        }
        Err(e) => return Err(e),
    };
    
    let mut buffer = [0u8; 8];
    file.read_exact(&mut buffer)?;
    
    Ok(Db {
        path: file_path,
        dimensions_amount: u64::from_le_bytes(buffer),
    })
}