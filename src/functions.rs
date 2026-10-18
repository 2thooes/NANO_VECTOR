use crate::command::Command;
use crate::db::Db;
use std::fs::{self, File};
use std::io;
use std::io::Read;
use directories::ProjectDirs;
use std::fs::OpenOptions;
use std::io::Write;

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
            "Not enough arguments",
        ));
    }

    if command.sarg.len() > 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Too many arguments",
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
    let mut content = Vec::new();
    let dimensions_amount: u64 = command.sarg[0] as u64;
    let mut content_length: u8 = 32;
    content.extend_from_slice(&(dimensions_amount).to_le_bytes());
    // Content length value by default is 32
    content.push(content_length);
    //if content length given, changing the default value
    if(command.sarg.len() == 2){
        println!("asasd");
        content_length = command.sarg[1] as u8;
        content[8] = (content_length);
    }
    fs::write(&file_path,&content)?;

    // 6. Returning the address
    Ok(Db{
        path: file_path,
        dimensions_amount: dimensions_amount,
        content_length: content_length,
    })
}
//DROPDB command
pub fn dropdb(command: &Command, db: &Db) -> io::Result<Db> {
    if !command.sarg.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Too many arguments",
        ));
    }

    let file_path = if !command.marg.is_empty() {
        let proj_dirs = ProjectDirs::from("com", "2thooes", "nano_vector")
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cannot identify the system folder"))?;
        let data_dir = proj_dirs.data_dir();
        data_dir.join(format!("{}.bin", command.marg))
    } else if *db != Db::default() {
        db.path.clone()
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Not enough arguments: db name is not specified",
        ));
    };

    if file_path.exists() {
        fs::remove_file(&file_path)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Db cannot be found in: {:?}", file_path),
        ));
    }

    Ok(Db::default())
}
//ENTERDB command
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
                format!("Database {} was not found in directory: {:?}", command.marg, data_dir),
            ));
        }
        Err(e) => return Err(e),
    };
    
    let mut buffer = [0u8; 8];
    file.read_exact(&mut buffer)?;
    let dimensions = u64::from_le_bytes(buffer);

    // recreating buffer
    let mut buffer = [0u8; 1];
    file.read_exact(&mut buffer)?;
    let content = u8::from_le_bytes(buffer);

    Ok(Db {
        path: file_path,
        dimensions_amount: dimensions/*u64::from_le_bytes(buffer)*/,
        content_length: content,
    })
}
//EXITDB command
pub fn exitdb(command: &Command, db: &Db) -> io::Result<Db> {
    if !command.marg.is_empty() || !command.sarg.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "Too many arguments"));
    }
    if db.dimensions_amount == 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "You are not in db"));
    }
    // Returning default value
    Ok(Db::default())
}
//LSDB command
pub fn lsdb(command: &Command, db: &Db) -> io::Result<Vec<String>>{
    let proj_dirs = ProjectDirs::from("com", "2thooes", "nano_vector")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cannot identify the system folder"))?;

    let data_dir = proj_dirs.data_dir();
    
    if !data_dir.exists() {
        return Ok(Vec::new());
    }

    let mut db_names = Vec::new();

    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name_os) = path.file_name() {
                if let Some(file_name_str) = file_name_os.to_str() {
                    // Cut .bin if needed
                    if let Some(name_without_ext) = file_name_str.strip_suffix(".bin") {
                        db_names.push(name_without_ext.to_string());
                    }
                }
            }
        }
    }

    Ok(db_names)
}
//PUT command
pub fn put(command: &Command, db: &Db) -> io::Result<String>{
    if *db == Db::default(){
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Put cannot be used outside of the database"),
        ));
    }
    if(command.marg.is_empty() || command.sarg.is_empty() || db.dimensions_amount > command.sarg.len().try_into().unwrap()){
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Not enough arguments"),
        ));
    }
    if db.dimensions_amount < command.sarg.len().try_into().unwrap(){
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Too many arguments"),
        ));
    }
    let length:usize = db.content_length as usize;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&db.path)?;//макс длинна стринга 64 байта, длина компонента вектора не фиксирована, заголовка 16

    let bytes = command.marg.as_bytes();
    if bytes.len() > length{
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Value is too long")
        ))
    }
    //value writing
    let mut buffer = vec![0u8;length];
    buffer[..bytes.len()].copy_from_slice(bytes);
    file.write_all(&buffer)?;

    //coordinates writing
    for coord in &command.sarg{
        file.write_all(&coord.to_le_bytes())?;
    }
    Ok("put done".to_string())
}
pub fn status(command: &Command){
    println!("Status was called")
}