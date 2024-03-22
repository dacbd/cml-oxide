use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}
#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    Runner {}
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Runner {} => {
            println!("runner not implemented yet!")
        }
    }
    Ok(())
}
