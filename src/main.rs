use ::clap::{Parser, Subcommand, Args};

mod core;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "strg - A persistent storage solution for docker containers",
    long_about = "A persistent storage solution that syncs database files located in a Docker container under your GitHub account"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync your Database files
    Sync(Sync),
}

#[derive(Args)]
struct Sync {
    /// Database Name
    db: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Sync(name)) => {
            match name.db {
                Some(ref _name) => {
                    let db = core::strg::sync(_name);
                    println!("{}", db);
                }

                None => {
                    println!("Please provide a database name");
                }
            }
        }

        None => {}
    }
}
