// builtins.rs
use std::vec::Vec;

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
        _ => {
            return (ReturnedEffect::NoMatch, Vec::new());
        }
    }
}
