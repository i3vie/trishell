#![feature(const_option)]

use std::env;
use std::io::{self, ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, exit};

use config::{get_config, parse_prompt};
use dirs::home_dir;
use regex::Regex;

mod builtins;
mod config;

fn main() {
    let home = home_dir().unwrap_or_else(|| PathBuf::from("/"));
    let home_str = home.to_str().unwrap();

    let config = get_config();

    let env_var_regex = Regex::new(r"\$(\w+)").unwrap();

    loop {

        let prompt = parse_prompt(&config);

        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        // match on the result of read_line
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // eof
                exit(0);
            }
            Ok(_) => {
                let input_raw = input.trim();

                let input = env_var_regex.replace_all(input_raw, |caps: &regex::Captures| {
                    let var_name = &caps[1];
                    env::var(var_name).unwrap_or_else(|_| "".to_string())
                }).to_string();

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
            }
        }
    }
}
