use std::fs;

use directories::ProjectDirs;

use crate::config::Theme;

macro_rules! template {
    () => {
        // TODO: Set variable selection_background
        "
# The basic colors
foreground            {foreground}
background            {background}
selection_foreground  {color15}
selection_background  #44475a

# URL underline color when hovering with mouse
url_color {color6}

# The 16 terminal colors

# black
color0  {color0}
color8  {color8}

# red
color1  {color1}
color9  {color9}

# green
color2  {color2}
color10 {color10}

# yellow
color3  {color3}
color11 {color11}

# blue
color4  {color4}
color12 {color12}

# magenta
color5  {color5}
color13 {color13}

# cyan
color6  {color6}
color14 {color14}

# white
color7  {color7}
color15 {color15}

# Cursor colors
cursor            {foreground}
cursor_text_color {background}

# Tab bar colors
active_tab_foreground   {background}
active_tab_background   {foreground}
inactive_tab_foreground {background}
inactive_tab_background {color8}

# Marks
mark1_foreground {background}
mark1_background {color1}

# Splits/Windows
active_border_color {color7}
inactive_border_color {color8}
"
    };
}

pub fn generate(theme: &Theme) -> std::io::Result<()> {
    let conf: String = format!(
        template!(),
        foreground = theme.foreground,
        background = theme.background,
        color0 = theme.color0,
        color1 = theme.color1,
        color2 = theme.color2,
        color3 = theme.color3,
        color4 = theme.color4,
        color5 = theme.color5,
        color6 = theme.color6,
        color7 = theme.color7,
        color8 = theme.color8,
        color9 = theme.color9,
        color10 = theme.color10,
        color11 = theme.color11,
        color12 = theme.color12,
        color13 = theme.color13,
        color14 = theme.color14,
        color15 = theme.color15
    );

    if let Some(proj_dirs) = ProjectDirs::from("", "", "theme") {
        let file_path = proj_dirs.data_dir().join("kitty-colors.conf");
        if let Some(p) = file_path.parent() {
            fs::create_dir_all(p)?
        };
        fs::write(file_path, conf)?;
    }
    Ok(())
}
