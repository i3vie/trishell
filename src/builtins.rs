// builtins.rs
use std::{env::{set_var, var}, path::{Path, PathBuf}, process::exit, vec::Vec};
use dirs::home_dir;   // We just use the crate because there are always edge cases
use path_absolutize::*;

pub enum ReturnedEffect {
    NoMatch,
    NoEffect
}

#[allow(unused_variables)]
pub fn parse_builtins(command: &str, args: &mut[&str]) -> (ReturnedEffect, Vec<String>) {
    match command {
        "cd" => {
            let home = home_dir().unwrap_or_else(|| PathBuf::from("/"));
            let home_str = home.to_str().unwrap();

            let mut temp_args: Vec<String> = Vec::new(); // FUCK

            args.iter().for_each(|&s| {
                let new_str = s.replace("~", home_str);
                temp_args.push(new_str); // FUUUUCK
            });

            let updated_args: Vec<&str> = temp_args.iter().map(|s| s.as_str()).collect(); // AAAAAAAAAA

            let mut cd_dir = if updated_args.is_empty() {
                home
            } else {
                Path::new(updated_args.join(" ").as_str()).to_path_buf()
            };

            cd_dir = cd_dir.absolutize().unwrap().into_owned();

            match std::env::set_current_dir(&cd_dir) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!(
                        "cd: {}: {}",
                        e.to_string().to_lowercase().split(" (os error").next().unwrap(),
                        cd_dir.display()
                    );
                }
            }
            set_var("PWD", &cd_dir);

            return (
                ReturnedEffect::NoEffect,
                Vec::new()
            );
        }
        "pwd" => {
            println!("{}", var("PWD").unwrap());
            return (ReturnedEffect::NoEffect, Vec::new());
        }
        "exit" => {
            exit(0);
        }
        _ => {
            return (ReturnedEffect::NoMatch, Vec::new());
        }
    }
}
