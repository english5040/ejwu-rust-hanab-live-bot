#[derive(clap::Parser)]
#[command(
    no_binary_name = true,
    bin_name = "/pm <BOT_NAME>",
    disable_help_subcommand = true
)]
pub enum ChatCommand {
    #[command(name = "/join")]
    Join {
        #[arg(short, long)]
        password: Option<String>,
    },
    #[command(name = "/leave")]
    Leave,
    #[command(name = "/create")]
    Create {
        #[arg(short, long)]
        table_name: Option<String>,
        #[arg(short, long, default_value_t = 6)]
        max_players: u8,
        #[arg(short, long)]
        password: Option<String>,
    },
    #[command(name = "/start")]
    Start,
}
