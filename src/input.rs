use crate::command::Command;
use std::io;

pub fn input_formatting(input: String) -> Command {
    // String cleaning
    let temp: Vec<&str> = input.split_whitespace().collect();

    // Change variables safely
    let body = temp.get(0).copied().unwrap_or("").to_string();
    let marg = temp.get(1).copied().unwrap_or("").to_string();
    let sarg = if temp.len() > 2 {
        temp[2..]
            .iter()
            .filter_map(|s| s.parse::<f32>().ok()) // Skip not valid values
            .collect()
    } else {
        vec![]
    };

    Command { body, marg, sarg }
}

pub fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the command");
    input
}
