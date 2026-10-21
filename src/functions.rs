use crate::command::Command;
use crate::db::Db;
use std::fs::{self, File};
use directories::ProjectDirs;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, Read, Seek, SeekFrom};

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
    let file = File::create(&file_path)?;
    let mut content = Vec::new();
    let dimensions_amount: u64 = command.sarg[0] as u64;
    let mut content_length: u8 = 32;
    content.extend_from_slice(&(dimensions_amount).to_le_bytes());
    // Content length value by default is 32
    content.push(content_length);
    //if content length given, changing the default value
    if command.sarg.len() == 2{
        println!("asasd");
        content_length = command.sarg[1] as u8;
        content[8] = content_length;
    }
    fs::write(&file_path,&content)?;

    // 6. Returning the address
    Ok(Db{
        path: file_path,
        file: Some(file),
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
    
    let dimensions = u64::from_le_bytes(get_bytes(&mut file,0,8).try_into().unwrap());//handle errors
    let content = u8::from_le_bytes(get_bytes(&mut file,8,1).try_into().unwrap());//handle errors

    Ok(Db {
        path: file_path,
        file: Some(file),
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
    if command.marg.is_empty() || command.sarg.is_empty() || db.dimensions_amount > command.sarg.len().try_into().unwrap(){
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
        .open(&db.path)?;//макс длинна стринга 64 бита, длина компонента вектора не фиксирована, заголовка 16

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
//GET command
pub fn get(command: &Command, db: &mut Db) -> io::Result<String>{
    if command.marg.is_empty(){
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Not enough arguments"),
        ))
    }
    if *db == Db::default(){
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("You are not in db"),
        ))
    }
    match command.marg.as_str(){
        "all" => return get_all(command,db),
        "byid" =>  return get_byid(command,db),
        _ => return Ok("Else was called".to_string()),
    }
    return Ok("".to_string());
}

fn get_all(command: &Command, db: &mut Db) -> io::Result<String>{
    println!("inside get_all");
    let metadata = fs::metadata(&db.path)?;
    let file_size: u64 = metadata.len();
    //println!("Total byte length: {}, amount of working bytes: {}", file_size,file_size-9);
    let step:usize = (db.content_length as u64 + 4 * db.dimensions_amount) as usize;
    for i in (9..file_size).step_by(step){
        println!("Element:{:?}",get_bytes(&mut db.file.as_mut().unwrap(),i,db.content_length as u64));
    }
    println!("inside get_all");
    return Ok("get_all called".to_string());
}

fn get_byid(command: &Command, db: &Db) -> io::Result<String>{
    return Ok("get_byid called".to_string());
}

fn get_bytes(file: &mut File,start: u64, amount: u64) -> Vec<u8>{
    file.seek(SeekFrom::Start(start));
    let mut buffer = vec![0u8;amount as usize];
    file.read_exact(&mut buffer);
    file.seek(SeekFrom::Start(0));
    //Ok(buffer);
    buffer
}

pub fn status(command: &Command){
    println!("Status was called")
}