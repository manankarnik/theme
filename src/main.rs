use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    color: String,
}

fn main() {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "theme") {
        let config_dir = proj_dirs.config_dir();
        let config_file = fs::read_to_string(config_dir.join("config.toml"));
        let config: Config = match config_file {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => Config {
                name: "Theme".to_string(),
                color: "Red".to_string(),
            },
        };
        dbg!(config);
    }
}
