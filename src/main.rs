mod command;
mod input;
mod functions;
mod db;
use crate::command::Command;
use crate::db::Db;
use std::io::{self, Write};

fn main() {
    println!("NANO_VECTOR by Tymur Mikhaliev\ninitok");
    let mut database:Db = Default::default(); 
    loop {
        invitation(&database);
        let command = input::input_formatting(input::input());
        show_command(&command);
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
                let result = functions::lsdb(&command,&database);
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
            "status" => functions::status(&command),
            _ => println!("Command not found"),
        }
        show_db(&database);
    }
}

fn show_command(command: &Command) {
    println!("body: <{}>", command.body);
    println!("marg: <{}>", command.marg);
    println!("sarg: {:?}", command.sarg);
}

fn show_db(db: &Db){
    println!("path: <{}>", db.path.display());
    println!("dim:  <{}>", db.dimensions_amount);
    println!("cont: {:?}", db.content_length);
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

