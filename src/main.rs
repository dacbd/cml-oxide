mod comment;
mod common;
//mod drivers;
mod pr;
mod publish;
mod runner;

use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::Serialize;
use tracing::{error, info};

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

#[derive(Debug, clap::ValueEnum, Clone, Serialize)]
enum Forge {
    Github,
    Gitlab,
    Unknown,
}
impl Forge {
    pub fn get_token(&self) -> Result<String, &str> {
        let native_token: Option<String> = match self {
            Self::Github => std::env::var("GITHUB_TOKEN").ok(),
            Self::Gitlab => std::env::var("GITLAB_TOKEN").ok(),
            Self::Unknown => None,
        };
        match native_token {
            Some(t) => Ok(t),
            None => match std::env::var("CML_TOKEN").or_else(|_| std::env::var("REPO_TOKEN")) {
                Ok(t) => Ok(t),
                Err(e) => {
                    error!(error = ?e, "Failed to load auth env variables");
                    Err("No token detected")
                }
            },
        }
    }
}
impl Default for Forge {
    fn default() -> Self {
        if let Ok(token) = std::env::var("CML_TOKEN").or_else(|_| std::env::var("REPO_TOKEN")) {
            // TODO: inspect token to determine forge
            let gh_prefixes = ["ghp", "gho", "ghu", "ghs", "ghr"];
            if gh_prefixes.iter().any(|&prefix| token.starts_with(prefix)) {
                return Self::Github;
            }
            return Self::Unknown;
        }
        if let Ok(_github_token) = std::env::var("GITHUB_TOKEN") {
            return Self::Github;
        }
        if let Ok(_gitlab_token) = std::env::var("GITLAB_TOKEN") {
            return Self::Gitlab;
        }
        return Self::Unknown;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let root_args = Cli::parse();
    // TODO: logging level from the cli args isnt work how I want.
    let logging_level = determine_log_level(root_args.verbose);
    let _ = tracing_subscriber::fmt().with_max_level(logging_level);

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
