mod command;
mod input;
mod functions;
mod db;
mod subfunctions;
mod helper_functions;
use crate::db::Db;
use std::io::{self, Write};

fn main() {
    println!("NANO_VECTOR by Tymur Mikhaliev\ninitok");
    let mut database:Db = Default::default(); 
    loop {
        invitation(&database);
        let command = input::input_formatting(input::input());
        //show_command(&command);
        match command.body.as_str(){
            "help" => functions::help(&command),
            "newdb" => {
                let result = functions::newdb(&command);
                match result {
                    Ok(db) => {
                        println!("Success!");
                        database = db;    
                    },
                    Err(e) => println!("Err: {}", e),
                }
            }
            "dropdb" => {
                let result = functions::dropdb(&command,&database);
                match result {
                    Ok(db) => {
                        println!("Success!");  
                        database = db;  
                    },
                    Err(e) => println!("Err: {}", e),
                }
            },
            "enterdb" =>{
                let result = functions::enterdb(&command,&database);
                match result {
                    Ok(db) => {
                        println!("Success!");  
                        database = db;  
                    },
                    Err(e) => println!("Err: {}", e),
                }
            }
            "exitdb" => {
                let result = functions::exitdb(&command,&database);
                match result {
                    Ok(db) => {
                        println!("Success!");
                        database = db;  
                    },
                    Err(e) => println!("Err: {}", e),
                }
            },
            "lsdb" => {
                let result = functions::lsdb();
                match result {
                    Ok(vector) => {
                        for name in &vector{
                            println!("Name:{}",name);
                        }
                    },
                    Err(e) => println!("Err: {}", e),
                }
            }
            "put" => {
                let result = functions::put(&command,&database);
                match result {
                    Ok(string) => {
                        println!("{}",string);
                    },
                    Err(e) => println!("Err: {}", e),
                }
            }
            "get" => {
                let result = functions::get(&command,&mut database);
                match result{
                    Ok(string) =>{
                        println!("{}",string);
                    },
                    Err(e) => println!("Err: {}", e),
                }
            }
            "closest" => {
                let result = functions::closest(&command,&mut database);
                match result{
                    Ok(num) =>{
                        println!("{}",num);
                    },
                    Err(e) => println!("Err: {}",e),
                }
            }
            "distance" => {
                let result = functions::distance(&command,&mut database);
                match result{
                    Ok(string) =>{
                        println!("{}",string);
                    },
                    Err(e) => println!("Err: {}",e),
                }
            }
            "status" => functions::status(&mut database),
            _ => println!("Command not found"),
        }
        //show_db(&database);
    }
}

fn invitation(db: &Db){
    print!("~");
    if *db !=Db::default(){
        let file_name = db.path.file_stem().and_then(|s| s.to_str()).unwrap();
        print!("[{}]",file_name);
    }
    print!("~");
    io::stdout().flush().unwrap();
}

