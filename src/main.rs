use clap::Parser;

mod config;
mod reload;
mod templates;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// Set theme
    Set { theme: String },
}

fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();
    match &args.command {
        Command::Set { theme } => {
            let theme: config::Theme = config::get_theme(theme);
            templates::terminals::kitty::generate(&theme)?;
            reload::kitty();
        }
    }
    Ok(())
}
