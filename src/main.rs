use std::io::{self, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    print!("$ ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    if parse_cmd(&input) {
        print!("Command found!");
    } else {
        print!("{}: command not found\n", input.trim_end());
    }
}

fn parse_cmd(_input: &String) -> bool {
    return false;
}