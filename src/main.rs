use std::io::{self, ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, exit};

use dirs::home_dir;

mod builtins;

#[allow(unused_variables)]
fn main() {
    let home = home_dir().unwrap_or_else(|| PathBuf::from("/"));
    let home_str = home.to_str().unwrap();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            exit(0);
        }

        let mut parts: Vec<&str> = input.split_whitespace().collect();
        if let Some((command, args)) = parts.split_first_mut() {

            let mut temp_args: Vec<String> = Vec::new(); // FUCK

            args.iter().for_each(|&s| {
                let new_str = s.replace("~", home_str);
                temp_args.push(new_str); // FUUUUCK
            });

            let updated_args: Vec<&str> = temp_args.iter().map(|s| s.as_str()).collect(); // AAAAAAAAAA

            let (effect, response) = builtins::parse_builtins(command, updated_args.as_slice());

            match effect {
                builtins::ReturnedEffect::NoEffect => {} // Shell main doesn't have to do anything
                builtins::ReturnedEffect::NoMatch => { // No matching builtin
                    let status = Command::new(&mut *command)
                        .args(updated_args)
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