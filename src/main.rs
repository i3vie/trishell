use std::env;
use std::io::{self, ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, exit};

mod builtins;

fn main() {

    let mut pwd: PathBuf = match env::current_dir() {
        Ok(pbuf) => pbuf,
        Err(_) => PathBuf::new(),
    };

    if pwd == PathBuf::new() {
        pwd.push("/");
    }

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            exit(0);
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if let Some((command, args)) = parts.split_first() {

            let (effect, response) = builtins::parse_builtins(command, &args);

            match effect {
                builtins::ReturnedEffect::ChangePath => {
                    // change the path based on whatever is in response
                    println!("builtin was {} with response {:?}", command, response)
                }
                builtins::ReturnedEffect::NoMatch => {
                    println!("no matching builtin found, trying binaries");

                    let status = Command::new(command)
                        .args(args)
                        .status();
    
                    if let Err(e) = status {
                        if e.kind() == ErrorKind::NotFound {
                            eprintln!("trishell: command not found: {}", command)
                        } else {
                            eprintln!("Failed to execute command: {}", e.kind());
                        }
                    }
                    
                }
            }
        }
    }
}