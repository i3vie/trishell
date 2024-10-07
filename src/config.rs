use std::{ffi::OsStr, fs};

use dirs::home_dir;
use users::get_current_username;
use yaml_rust::{Yaml, YamlLoader};

extern crate yaml_rust;
extern crate dirs;

#[allow(dead_code)] // This isn't dead?
const DEFAULT_CONFIG: &str = "

# $U is the username
# $H is the hostname
# $D is the current absolute directory
# $d is the current directory name only
# $$ is a literal $

prompt_format: \"$U@$H [$D] $$ \"

";

pub fn get_config() -> Yaml { // TODO: Function to create the directory, file and dump the default config into it
    let mut config_file = dirs::config_dir().unwrap();
    config_file.push("./trishell/config.yml");

    let contents = fs::read_to_string(&config_file).or::<String>(Ok(DEFAULT_CONFIG.to_string())).unwrap();

    let documents = YamlLoader::load_from_str(contents.as_str()).unwrap();
    let config = &documents[0];

    config.clone()
}

pub fn parse_prompt(config: &Yaml) -> String {
    let prompt_format = config["prompt_format"].as_str().unwrap();

    let current_dir = std::env::current_dir().unwrap();
    let home = home_dir().unwrap();
    let home_string = home.to_str().unwrap();

    let mut full_dir = String::from(current_dir.as_os_str().to_str().unwrap());

    let mut front_dir = String::from(
        current_dir
            .file_name()
            .unwrap_or(OsStr::new("/"))
            .to_str()
            .unwrap()
    );

    full_dir = full_dir.replace(&home_string, "~");
    front_dir = if &home_string == &current_dir.to_str().unwrap() {
        String::from("~")
    } else {
        front_dir
    };

    let mut prompt = prompt_format.replace("$U", get_current_username().unwrap().to_str().unwrap());
    prompt = prompt.replace("$H", fs::read_to_string("/proc/sys/kernel/hostname").unwrap().trim());
    prompt = prompt.replace("$D", &full_dir);
    prompt = prompt.replace("$d", &front_dir);
    prompt = prompt.replace("$$", "$");

    prompt
}