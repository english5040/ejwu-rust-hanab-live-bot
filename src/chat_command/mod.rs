use clap::Parser;

#[derive(clap::Parser)]
#[command(no_binary_name = true)]
pub struct Join {
    #[arg(short, long)]
    password: Option<String>,
}

#[derive(clap::Parser)]
#[command(no_binary_name = true)]
pub struct Leave;

#[derive(clap::Parser)]
#[command(no_binary_name = true)]
pub struct Create {
    #[arg(short, long)]
    table_name: Option<String>,
    #[arg(short, long, default_value_t = 6)]
    max_players: u8,
    #[arg(short, long)]
    password: Option<String>,
}

#[derive(clap::Parser)]
#[command(no_binary_name = true)]
pub struct Start;
