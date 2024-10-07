use std::fs;

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

prompt_format: \"$U@$H $$ \"

";

pub fn get_config() -> Yaml {
    let mut config_file = dirs::config_dir().unwrap();
    config_file.push("./trishell/config.yml");

    let contents = fs::read_to_string(&config_file).or::<String>(Ok(DEFAULT_CONFIG.to_string())).unwrap();

    let documents = YamlLoader::load_from_str(contents.as_str()).unwrap();
    let config = &documents[0];

    config.clone()
}

pub fn parse_prompt(config: &Yaml) -> String {
    let prompt_format = config["prompt_format"].as_str().unwrap();

    let mut prompt = prompt_format.replace("$U", get_current_username().unwrap().to_str().unwrap());
    prompt = prompt.replace("$H", fs::read_to_string("/proc/sys/kernel/hostname").unwrap().trim());
    prompt = prompt.replace("$D", std::env::current_dir().unwrap().as_os_str().to_str().unwrap());
    prompt = prompt.replace("$$", "$");

    prompt
}