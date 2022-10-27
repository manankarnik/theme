use clap::Parser;

mod config;

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

fn main() {
    let args: Args = Args::parse();
    match &args.command {
        Command::Set { theme } => {
            let theme: config::Theme = config::get_theme(theme);
            dbg!(theme);
        }
    }
}
