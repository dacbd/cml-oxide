use clap::Parser;

#[derive(Debug, Parser)]
#[command()]
pub struct Args {
    #[arg(short, long)]
    title: String,
    #[arg(short, long, default_value_t = true)]
    native: bool,
    #[arg(short, long, default_value_t = true)]
    watermark: bool,
    #[arg(long)]
    mime_type: Option<String>,
    #[arg(short, long)]
    file: String,
}
