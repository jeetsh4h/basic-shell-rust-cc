use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        parse_cmd(trimmed_input);
    }
}

const BUILT_IN_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

fn parse_cmd(input: &str) {
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    if parts.len() == 0 {
        return;
    }

    match parts[0] {
        "exit" => {
            if parts.len() != 2 {
                print!("exit code not provided\n");
                return;
            }
            match parts[1].parse::<i32>() {
                Ok(code) => std::process::exit(code),
                Err(_) => {
                    print!("exit code not an integer\n");
                    return;
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
            return;
        }
        "type" => {
            if parts.len() != 2 {
                print!("type: missing argument\n");
                return;
            }
            let cmd = parts[1];
            if BUILT_IN_COMMANDS.contains(&cmd) {
                print!("{cmd} is a shell builtin\n");
                return;
            }
            match env::var("PATH") {
                Err(_) => {
                    print!("type: cannot find PATH\n");
                    return;
                }
                Ok(paths) => {
                    for path in paths.split(":") {
                        let full_path = format!("{path}/{cmd}");
                        if std::path::Path::new(&full_path).exists() {
                            print!("{cmd} is {full_path}\n");
                            return;
                        }
                    }
                    print!("{cmd}: not found\n");
                    return;
                }
            }
        }
        _ => match env::var("PATH") {
            Err(_) => {
                print!("PATH not found\n");
                return;
            }
            Ok(paths) => {
                for path in paths.split(":") {
                    let full_path = format!("{}/{}", path, parts[0]);
                    if std::path::Path::new(&full_path).exists() {
                        let mut child = Command::new(full_path)
                            .args(parts[1..].iter())
                            .spawn()
                            .expect("failed to execute command");

                        let _result = child.wait().expect("failed to wait on child");
                        return;
                    }
                }
                print!("{}: command not found\n", parts[0]);
                return;
            }
        },
    }
}
