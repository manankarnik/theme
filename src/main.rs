mod config;

fn main() {
    let config: config::Config = config::get_config();
    dbg!(config);
}
