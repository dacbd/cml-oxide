use clap::Parser;

#[derive(Debug, Parser)]
#[command()]
pub struct Args {
    /// Path to the file which will be comment's body/text
    file: String,
    /// Comment type, ex: commit, pr, issue, commit/1337dacb, pr/42, issue/4237
    /// default is pr, but will fall back to a commit comment
    #[arg(short, long)]
    target: Option<String>,
    #[arg(long, default_value_t = true)]
    watermark: bool,
}
