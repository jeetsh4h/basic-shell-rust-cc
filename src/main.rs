use std::io::{self, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if !parse_cmd(&input) {
            print!("{}: command not found\n", input.trim());
        }
    }
}

fn parse_cmd(input: &String) -> bool {
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    if parts.len() == 0 {
        return true;
    }

    match parts[0] {
        "exit" => {
            if parts.len() != 2 {
                print!("exit code not provided\n");
                return true;
            }
            std::process::exit(parts[1].parse::<i32>().unwrap());
        }
        _ => return false,
    }
}
