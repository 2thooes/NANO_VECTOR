use crate::command::Command;

pub fn help(command: &Command){
    if(command.marg.is_empty() && command.sarg.is_empty()){
        println!("Help was called");
    }
    else{
        println!("Err:Too many arguments");
    }
}

