#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::wildcard_imports)]

mod command;
mod hanabi_client;

use std::collections::HashMap;

use clap::Parser;
use color_eyre::eyre;
use command::client;
use futures::{prelude::*, stream::FuturesOrdered};
use serde::Deserialize;

use crate::hanabi_client::Bot;

// TODO remove futures dependency- just use tokio's

#[derive(clap::Parser)]
struct Args {
    // Number of bots to run. Will use default usernames.
    #[arg(
        short,
        default_value_t = 1,
        value_parser = clap::value_parser!(u8).range(1..=6),
        group = "users"
    )]
    n: u8,
    // Usernames of all bots to run
    #[arg(long, value_name = "USERNAME", group = "users")]
    user: Option<Vec<String>>,
    // Whether to create a table. If table not set, use a random name
    // When create is true, bots will join the first bot's table automatically
    #[arg(short, long, group = "create_or_join")]
    create: bool,
    // Table to create or table to join
    #[arg(long, value_name = "NAME", group = "join_target")]
    table: Option<String>,
    // User to join. Named follow_user because we'll try to join whichever table
    // that user goes to.
    #[arg(
        long,
        value_name = "USERNAME",
        group = "create_or_join",
        group = "join_target"
    )]
    follow_user: Option<String>,
}

#[derive(Deserialize)]
struct Config {
    // Bot username, password
    bots: HashMap<String, String>,
    // List of default bot usernames
    default_bots: Vec<String>,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().init();

    // Synchronous
    let f = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&f)?;

    let args = Args::parse();

    let bot_usernames: Vec<&str> = if let Some(users) = &args.user {
        users.iter().map(|s| &**s).collect()
    } else {
        config.default_bots[0..args.n.into()]
            .iter()
            .map(|s| &**s)
            .collect()
    };
    let bot_usernames_passwords = bot_usernames
        .iter()
        .map(|&username| (username, &*config.bots[username]));

    let (bots, futures) = run_bots(bot_usernames_passwords).await?;

    // Process args
    if args.create {
        let table = client::TableCreate {
            name: args.table,
            ..client::TableCreate::default()
        };
        bots[0].create_table(table);
        for bot in &bots[1..] {
            todo!()
        }
    } else if let Some(user) = args.follow_user {
        for bot in bots {
            todo!()
        }
    } else if let Some(name) = args.table {
        for bot in bots {
            todo!()
        }
    }

    let results = futures.collect::<Vec<_>>().await;
    dbg!(results);
    Ok(())
}

// Make and run bots
// Returns a Vec<Bot>, and a FuturesOrdered containing the bots' JoinHandles
#[allow(clippy::future_not_send)]
async fn run_bots<'a, 'b, T>(
    bot_username_password: T,
) -> eyre::Result<(
    Vec<Bot>,
    FuturesOrdered<impl Future<Output = eyre::Result<()>>>,
)>
where
    T: Iterator<Item = (&'a str, &'b str)>,
{
    let bots = bot_username_password
        .map(|(username, password)| Bot::new(username, password))
        .collect::<FuturesOrdered<_>>()
        .try_collect::<Vec<_>>()
        .await?;
    let (bots, futures): (Vec<_>, FuturesOrdered<_>) = bots.into_iter().unzip();
    Ok((bots, futures))
}
