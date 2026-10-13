use std::io;

struct Command{
    body: String,
    marg: String,
    sarg: Vec<i32>,
}

fn main() {
    println!("NANO_VECTOR by Tymur Mikhaliev\ninitok");
    loop{
        let command = input_formatting(input());
        show_command(command);show_command(command);
    }
}

fn input_formatting(mut input:String) -> Command{
    input = input.trim().to_string();
    let mut temp: Vec<&str> = input.split(' ').collect();
    let mut n:usize = 0;
    while n<temp.len(){
        if temp[n] == " " || temp[n] == ""{
            temp.remove(n);
        }
        else{
            n+=1;
        }        
    }
    Command {
        body: temp[0].to_string(),
        marg: temp[1].to_string(),
        sarg: temp.drain(2..)
            .filter_map(|s| s.parse::<i32>().ok()) // during the conversion, if sarg conversion had error, the program will just skip
            .collect(),
    }
}

fn input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the command");
    input
}

fn show_command(command:Command){
    println!("body: <{}>", command.body);
    println!("marg: <{}>", command.marg);
    println!("sarg: {:?}", command.sarg);
}