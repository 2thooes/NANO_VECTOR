use crate::command::Command;
use crate::db::Db;
use std::fs::{self, File};
use std::io;
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
pub fn newdb(command: &Command) -> io::Result<PathBuf> {
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
        //println!("Err: Db with name '{}' already exists!", command.marg);
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Db with this name already exists",
        ));
    }

    // 5. Creating empty file
    let _file = File::create(&file_path)?;
    let header = command.sarg[0] as u64;
    fs::write(&file_path,header.to_le_bytes())?;
    
    println!("Db sucessfully created, path: {:?}", file_path);

    // 6. Returning the address
    Ok(file_path)
}