//use std::io;
mod command;
mod input;
mod functions;
//use functions::*;
use crate::command::Command;
//use input::*;

fn main() {
    println!("NANO_VECTOR by Tymur Mikhaliev\ninitok");
    loop {
        let command = input::input_formatting(input::input());
        show_command(&command);
        match command.body.as_str(){
            "help" => functions::help(&command),
            "newdb" => {
                let i = functions::newdb(&command);
                match i {
                    Ok(path) => println!("Success: {}", path.display()),
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

