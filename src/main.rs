mod command;
mod input;
mod functions;
mod db;
use crate::command::Command;
use crate::db::Db;

fn main() {
    println!("NANO_VECTOR by Tymur Mikhaliev\ninitok");
    let mut database:Db = Default::default(); 
    loop {
        let command = input::input_formatting(input::input());
        show_command(&command);
        match command.body.as_str(){
            "help" => functions::help(&command),
            "newdb" => {
                let i = functions::newdb(&command);
                match i {
                    Ok(db) => {
                        println!("Db sucessfully created, path: {}", db.path.display());
                        database = db;    
                    },
                    Err(e) => println!("Err: {}", e),
                }
            }
            _ => println!("Command not found"),
        }
    }
}

fn show_command(command: &Command) {
    println!("body: <{}>", command.body);
    println!("marg: <{}>", command.marg);
    println!("sarg: {:?}", command.sarg);
}

