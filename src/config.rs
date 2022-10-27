use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct Config {
    themes: Vec<Theme>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Theme {
    name: String,
    foreground: String,
    background: String,
    color0: String,
    color1: String,
    color2: String,
    color3: String,
    color4: String,
    color5: String,
    color6: String,
    color7: String,
    color8: String,
    color9: String,
    color10: String,
    color11: String,
    color12: String,
    color13: String,
    color14: String,
    color15: String,
}

pub fn get_theme(theme_name: &str) -> Theme {
    let default_theme: Theme = Theme {
        name: "Default".to_string(),
        foreground: "#d0d0d0".to_string(),
        background: "#151515".to_string(),
        color0: "#151515".to_string(),
        color1: "#ac4142".to_string(),
        color2: "#90a959".to_string(),
        color3: "#f4bf75".to_string(),
        color4: "#6a9fb5".to_string(),
        color5: "#aa759f".to_string(),
        color6: "#75b5aa".to_string(),
        color7: "#75b5aa".to_string(),
        color8: "#505050".to_string(),
        color9: "#ac4142".to_string(),
        color10: "#90a959".to_string(),
        color11: "#f4bf75".to_string(),
        color12: "#6a9fb5".to_string(),
        color13: "#aa759f".to_string(),
        color14: "#75b5aa".to_string(),
        color15: "#75b5aa".to_string(),
    };
    let config: Config;
    if let Some(proj_dirs) = ProjectDirs::from("", "", "theme") {
        let config_dir = proj_dirs.config_dir();
        let config_file = fs::read_to_string(config_dir.join("config.toml"));

        match config_file {
            // TODO: Fix panic if fields are missing from config
            Ok(file) => config = toml::from_str(&file).unwrap(),
            // Config file does not exist
            Err(_) => {
                println!("config.toml does not exist, generating default");
                return default_theme;
            }
        };
    } else {
        // TODO: Figure out why this is unreachable
        println!("$HOME/.config/theme directory does not exist, generating default");
        return default_theme;
    }
    for t in config.themes.iter() {
        if t.name == theme_name {
            return t.clone();
        }
    }
    println!("Theme not found in config, generating default");
    return default_theme;
}
