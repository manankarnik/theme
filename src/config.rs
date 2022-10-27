use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    name: String,
    color: String,
}

pub fn get_config() -> Config {
    let default_config: Config = Config {
        name: "Theme".to_string(),
        color: "Red".to_string(),
    };
    let config: Config;
    if let Some(proj_dirs) = ProjectDirs::from("", "", "theme") {
        let config_dir = proj_dirs.config_dir();
        let config_file = fs::read_to_string(config_dir.join("config.toml"));

        config = match config_file {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => default_config,
        };
    } else {
        config = default_config;
    }
    return config;
}
