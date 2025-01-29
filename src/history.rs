use std::{fs, path::PathBuf, sync::Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref HISTORY: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn history_path() -> PathBuf {
    let mut history_file = dirs::home_dir().unwrap();
    history_file.push("./.trishell_history");

    history_file
}

pub fn read_history() -> Vec<String> {
    let history_file = history_path();
    let contents = fs::read_to_string(&history_file).unwrap_or(String::from("")).trim().to_string();

    let mut history = HISTORY.lock().unwrap();
    *history = contents.lines().map(|s| s.to_string()).collect();
    history.clone()
}

pub fn add_to_history(command: &str) {
    if command.trim().is_empty() {
        return;
    }

    // Read from the file to ensure the latest history is loaded.
    let mut history = read_history();
    history.push(command.to_string());

    // Update the global HISTORY
    let mut history_lock = HISTORY.lock().unwrap();
    *history_lock = history.clone();

    let history_file = history_path();
    fs::write(&history_file, history.join("\n")).unwrap();
}


/**
 * This will get the <index>th command from the history, starting from
 * the most recent and going back in time.
 */
pub fn get_history_index(index: usize) -> Option<String> {
    if index == 0 {
        return None;
    }

    let history = HISTORY.lock().unwrap();
    history.iter().rev().nth(index - 1).cloned()
}

pub fn longest_entry() -> usize {
    let history = HISTORY.lock().unwrap();
    history.iter().map(|s| s.len()).max().unwrap_or(0)
}