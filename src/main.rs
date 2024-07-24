mod comment;
mod common;
//mod drivers;
mod error;
mod pr;
mod publish;
mod runner;

use anyhow::Result;
use clap::{Parser, Subcommand};
use common::forge::Forge;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Logging level
    #[arg(short, long, default_value_t = 2, action = clap::ArgAction::Count)]
    verbose: u8,
    #[arg(long, value_enum, default_value_t = Forge::default() )]
    forge: Forge,
}
#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    Runner(runner::Args),
    #[command()]
    Publish(publish::Args),
    #[command()]
    Comment(comment::Args),
    #[command()]
    PR(pr::Args),
}

#[tokio::main]
async fn main() -> Result<()> {
    let root_args = Cli::parse();
    // TODO: logging level from the cli args isnt work how I want.
    let logging_level = determine_log_level(root_args.verbose);
    tracing_subscriber::fmt()
        .with_max_level(logging_level)
        .init();

    match root_args.command {
        Commands::Runner(args) => {
            println!("Runner not implemented yet {:?}", args)
        }
        Commands::Publish(args) => {
            println!("Publish not implemented yet {:?}", args)
        }
        Commands::Comment(args) => {
            println!("Comment not implemented yet {:?}", args);
            let payload = comment::Comment::from_args(&args);
            println!("{:?}", payload);
            payload.send(root_args.forge).await;
        }
        Commands::PR(args) => {
            println!("PR not implemented yet {:?}", args)
        }
    }
    Ok(())
}

#[tracing::instrument]
fn determine_log_level(verbose_count: u8) -> tracing::Level {
    match verbose_count {
        0 => tracing::Level::ERROR,
        1 => tracing::Level::WARN,
        2 => tracing::Level::INFO,
        3 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    }
}
