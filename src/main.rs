use std::collections::HashMap;
use std::env;
use std::io::{self, ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, exit};
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

use config::{get_config, parse_prompt, create_config_file};
use dirs::home_dir;
use regex::Regex;

mod builtins;
mod config;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--version" {
        println!("trishell {}", VERSION);
        return;
    }

    create_config_file();

    let exe_path = env::current_exe().unwrap();
    env::set_var("SHELL", exe_path);

    let home = home_dir().unwrap_or_else(|| PathBuf::from("/"));
    let home_str = home.to_str().unwrap();

    let config = get_config();

    let sub_env_var_regex = Regex::new(r"\$(\w+)").unwrap();

    let single_word_env_var_regex = Regex::new(r#"(?P<key>[A-Za-z_][A-Za-z0-9_]*)=(?P<value>[^ ]+)"#).unwrap();
    let multiword_env_var_regex = Regex::new(r#"(?P<key>[A-Za-z_][A-Za-z0-9_]*)=["'](?P<value>[^"']*)["']"#).unwrap();

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    loop {
        let prompt = parse_prompt(&config);

        print!("{}", prompt);
        stdout.flush().unwrap();

        let mut input = String::new();
        for c in stdin.lock().keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    print!("\r");
                    break;
                },
                Key::Char('\t') => {
                    // TODO(i3vie): handle tab completion here
                    print!("{}", '\x07');
                },
                Key::Ctrl('c') => {
                    // TODO(i3vie): uuuuuuuuuuuuuu
                    break;
                }
                Key::Ctrl('d') => {
                    print!("\r\n");
                    stdout.suspend_raw_mode().unwrap();
                    exit(0);
                }
                Key::Char(c) => {
                    input.push(c);
                    print!("{}", c);
                    stdout.flush().unwrap();
                }
                Key::Backspace => {
                    if (input.len()) > 0 {
                        input.pop();
                        print!("\x08 \x08");
                        stdout.flush().unwrap();
                    }
                }
                _ => {}
            }
        }

        let input_raw = input.trim();

        let mut input = sub_env_var_regex.replace_all(input_raw, |caps: &regex::Captures| {
            let var_name = &caps[1];
            env::var(var_name).unwrap_or_else(|_| "".to_string())
        }).to_string();

        let mut env_vars = HashMap::new();

        for caps in multiword_env_var_regex.captures_iter(&input) {
            env_vars.insert(caps["key"].to_string(), caps["value"].to_string());
        }
        input = multiword_env_var_regex.replace_all(&input, "").trim().to_string();

        for caps in single_word_env_var_regex.captures_iter(&input) {
            env_vars.insert(caps["key"].to_string(), caps["value"].to_string());
        }
        input = single_word_env_var_regex.replace_all(&input, "").trim().to_string();

        if input == "exit" {
            print!("\r\n");
            stdout.suspend_raw_mode().unwrap();
            exit(0);
        }

        let mut parts: Vec<String> = shell_words::split(&input).unwrap();
        if let Some((command, args)) = parts.split_first_mut() {
            let mut temp_args: Vec<String> = Vec::new();

            args.iter().for_each(|s| {
                let new_str = s.replace("~", home_str);
                temp_args.push(new_str);
            });

            let updated_args: Vec<&str> = temp_args.iter().map(|s| s.as_str()).collect(); 

            let (effect, _response) = builtins::parse_builtins(command, updated_args.as_slice());

            match effect {
                builtins::ReturnedEffect::NoEffect => {}
                builtins::ReturnedEffect::NoMatch => {
                    print!("\r\n");
                    stdout.suspend_raw_mode().unwrap();
                    let status = Command::new(&mut *command)
                        .args(&updated_args)
                        .envs(&env_vars)
                        .status();

                    if let Err(e) = status {
                        if e.kind() == ErrorKind::NotFound {
                            eprintln!("trishell: command not found: {}", command);
                        } else {
                            eprintln!("Failed to execute command: {}", e.kind());
                        }
                    }
                    stdout.activate_raw_mode().unwrap();
                }
            }
        }
        print!("\r\n");
    }
}