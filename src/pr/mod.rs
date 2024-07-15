use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command()]
pub struct Args {
    /// Pull/Merge Request title
    #[arg(short, long)]
    title: Option<String>,
    /// Body/description of the Pull Request
    #[arg(short, long)]
    body: Option<String>,
    /// The commit message
    #[arg(short, long)]
    message: Option<String>,
    /// To output in markdown format [](url)
    #[arg(long, default_value_t = false)]
    md: bool,
    /// The branch targeted by the pull request
    #[arg(long)]
    target_branch: Option<String>,
    /// the git user's name for the created commit
    #[arg(long)]
    git_user_name: Option<String>,
    /// the git user's email for the create commit
    #[arg(long)]
    git_user_email: Option<String>,

    #[arg(long, value_enum, default_value_t = MergeStyle::Squash)]
    merge_style: MergeStyle,
    #[arg(long, default_value_t = true)]
    watermark: bool,
    #[arg(long, default_value_t = false)]
    skip_ci: bool,
}

#[derive(Debug, clap::ValueEnum, Clone, Default, Deserialize, Serialize)]
enum MergeStyle {
    #[default]
    Squash,
    Rebase,
    Merge,
}
