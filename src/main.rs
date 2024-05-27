use std::io::{self, Write};
use std::env;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        if !parse_cmd(trimmed_input) {
            print!("{trimmed_input}: command not found\n");
        }
    }
}

const BUILT_IN_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

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
                print!("{echo_str}\n");
            }
            return true;
        }
        "type" => {
            if parts.len() != 2 {
                print!("type: missing argument\n");
                return true;
            }
            let cmd = parts[1];
            if BUILT_IN_COMMANDS.contains(&cmd) {
                print!("{cmd} is a shell builtin\n");
                return true;
            }
            match env::var("PATH") {
                Err(_) => {
                    print!("type: cannot find PATH\n");
                    return true;
                }
                Ok(paths) => {
                    for path in paths.split(":") {
                        let full_path = format!("{path}/{cmd}");
                        if std::path::Path::new(&full_path).exists() {
                            print!("{cmd} is {full_path}\n");
                            return true;
                        }
                    }
                    print!("{cmd}: not found\n");
                    return true;
                }
            }
        }
        _ => return false,
    }
}
