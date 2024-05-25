mod runner;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    Runner(runner::Args),
    #[command()]
    Publish {},
    #[command()]
    Comment {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Runner(args) => {
            println!("Runner not implemented yet {:?}", args)
        }
        Commands::Publish {} => {
            println!("Publish not implemented yet")
        }
        Commands::Comment {} => {
            println!("Comment not implemented yet")
        }
    }
    Ok(())
}
