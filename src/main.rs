use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        if !parse_cmd(trimmed_input) {
            print!("{}: command not found\n", trimmed_input);
        }
    }
}

fn parse_cmd(input: &str) -> bool {
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
            match parts[1].parse::<i32>() {
                Ok(code) => std::process::exit(code),
                Err(_) => {
                    print!("exit code not an integer\n");
                    return true;
                }
            }
        }
        "echo" => {
            if input.len() == 4 {
                print!("\n");
            } else {
                let echo_str = input[5..].trim();
                print!("{}\n", echo_str);
            }
            return true;
        }
        _ => return false,
    }
}
