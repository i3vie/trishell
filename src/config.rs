use std::fs;

use yaml_rust::{Yaml, YamlLoader};

extern crate yaml_rust;
extern crate dirs;

#[allow(dead_code)] // This isn't dead?
const DEFAULT_CONFIG: &str = "

# $U is the username
# $H is the hostname
# $D is the current dir
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