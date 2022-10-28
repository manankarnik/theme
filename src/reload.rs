use std::process::Command;

use directories::ProjectDirs;

pub fn kitty() {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "theme") {
        let file_path = proj_dirs.data_dir().join("kitty-colors.conf");
        Command::new("kitty")
            .args(&[
                "@",
                "set-colors",
                "--all",
                &file_path.clone().into_os_string().into_string().unwrap(),
            ])
            .output()
            .expect("Error reloading kitty");
    }
}
