use std::io::{self, ErrorKind, Write};
use std::process::{Command, exit};

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

        let parts: Vec<&str> = input.split_whitespace().collect();
        if let Some((command, args)) = parts.split_first() {
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