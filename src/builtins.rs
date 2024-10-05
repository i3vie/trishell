// builtins.rs
use std::{process::exit, vec::Vec};

pub enum ReturnedEffect {
    ChangePath,
    NoMatch,
}

pub fn parse_builtins(command: &str, args: &[&str]) -> (ReturnedEffect, Vec<String>) {
    match command {
        "cd" => {
            // Here you might want to handle the path argument if necessary
            return (ReturnedEffect::ChangePath, Vec::new());
        }
        "exit" => {
            exit(0);
        }
        _ => {
            return (ReturnedEffect::NoMatch, Vec::new());
        }
    }
}
