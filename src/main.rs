mod config;

fn main() {
    let theme: config::Theme = config::get_theme("Monokai Dark");
    dbg!(theme);
}
