#![feature(const_option)]

use std::fs;
use std::io::{self, ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, exit};
use users::get_current_username;

use config::get_config;
use dirs::home_dir;

mod builtins;
mod config;

fn main() {
    let home = home_dir().unwrap_or_else(|| PathBuf::from("/"));
    let home_str = home.to_str().unwrap();

    let config = get_config();

    loop {
        let prompt_format = config["prompt_format"].as_str().unwrap();

        let mut prompt = prompt_format.replace("$U", get_current_username().unwrap().to_str().unwrap());
        prompt = prompt.replace("$H", fs::read_to_string("/proc/sys/kernel/hostname").unwrap().trim());
        prompt = prompt.replace("$D", std::env::current_dir().unwrap().as_os_str().to_str().unwrap());
        prompt = prompt.replace("$$", "$");

        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        // Check the result of read_line
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // Detected Ctrl-D (EOF)
                exit(0);
            }
            Ok(_) => {
                let input = input.trim();

                if input == "exit" {
                    exit(0);
                }

                let mut parts: Vec<&str> = input.split_whitespace().collect();
                if let Some((command, args)) = parts.split_first_mut() {
                    let mut temp_args: Vec<String> = Vec::new();

                    args.iter().for_each(|&s| {
                        let new_str = s.replace("~", home_str);
                        temp_args.push(new_str);
                    });

                    let updated_args: Vec<&str> = temp_args.iter().map(|s| s.as_str()).collect(); 

                    let (effect, _response) = builtins::parse_builtins(command, updated_args.as_slice());

                    match effect {
                        builtins::ReturnedEffect::NoEffect => {}
                        builtins::ReturnedEffect::NoMatch => { 
                            let status = Command::new(&mut *command)
                                .args(updated_args)
                                .status();

                            if let Err(e) = status {
                                if e.kind() == ErrorKind::NotFound {
                                    eprintln!("trishell: command not found: {}", command);
                                } else {
                                    eprintln!("Failed to execute command: {}", e.kind());
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                // You might want to handle specific errors here if needed
            }
        }
    }
}
