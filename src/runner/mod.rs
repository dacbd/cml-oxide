use clap::Parser;
use nanoid::nanoid;

const ID_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

#[derive(Debug, clap::ValueEnum, Clone, Default)]
enum Modes {
    #[default]
    Single,
    Reuse,
    Idle,
}

#[derive(Debug, Parser)]
#[command()]
pub struct Args {
    #[arg(long, default_value_t = format!("cml-{}", nanoid!(10, &ID_CHARS)))]
    name: String,
    #[arg(long, default_values_t = vec![String::from("cml")])]
    labels: Vec<String>,
    #[arg(long, default_value = "single")]
    mode: Modes,
    #[arg(long, default_value_t = 300)]
    idle_timeout: usize,
    #[arg(long)]
    network_id: Option<String>,
    #[arg(long)]
    subnet_id: Option<String>,
    #[arg(long)]
    firewall_id: Option<String>,
    #[arg(long)]
    permission_set: Option<String>,
}
