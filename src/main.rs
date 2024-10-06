use std::io::{self, ErrorKind, Write};
use std::process::{Command, exit};

mod builtins;

#[allow(unused_variables)]
fn main() {

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
            let mut args = args;
            let (effect, response) = builtins::parse_builtins(command, &mut args);

            match effect {
                builtins::ReturnedEffect::NoEffect => {} // Shell main doesn't have to do anything
                builtins::ReturnedEffect::NoMatch => { // No matching builtin
                    let status = Command::new(&mut *command)
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