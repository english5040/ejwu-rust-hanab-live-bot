#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::wildcard_imports)]

mod command;
mod hanabi_client;

use std::collections::HashMap;

use clap::Parser;
use color_eyre::eyre;
use command::client;
use futures::{
    prelude::*,
    stream::{FuturesOrdered, FuturesUnordered},
};
use serde::Deserialize;

use crate::hanabi_client::Bot;

// TODO remove futures dependency- just use tokio's

#[derive(clap::Parser)]
struct Args {
    // Number of bots to run. Will use default usernames.
    #[arg(
        short,
        default_value_t = 1,
        value_parser = clap::builder::RangedU64ValueParser::<usize>::new().range(1..=6),
        group = "users"
    )]
    n: usize,
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

    let bot_username_passwords = if let Some(users) = &args.user {
        users.as_slice()
    } else {
        &config.default_bots[0..args.n]
    }
    .iter()
    .map(|s| &**s)
    .map(|username| (username, &*config.bots[username]));

    let (bots, mut futures) = run_bots(bot_username_passwords).await?;

    // Process args
    if args.create {
        bots[0].create_table(&client::TableCreate {
            name: args.table,
            ..client::TableCreate::default()
        });
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

    while let Some((i, username, result)) = futures.next().await {
        match result {
            Ok(()) => {
                tracing::info!("bots[{i}]{{username={username:?}}} finished");
            }
            Err(e) => {
                tracing::error!("bots[{i}]{{username={username:?}}} terminated with error: {e}");
            }
        }
    }
    Ok(())
}

// Make and run bots
// Returns a Vec<Bot>, and a FuturesUnordered containing the bots' JoinHandles
#[allow(clippy::future_not_send)]
async fn run_bots<'a, 'b, T>(
    bot_username_passwords: T,
) -> eyre::Result<(
    Vec<Bot>,
    FuturesUnordered<impl Future<Output = (usize, String, eyre::Result<()>)>>,
)>
where
    T: Iterator<Item = (&'a str, &'b str)>,
{
    let bots = bot_username_passwords
        .map(|(username, password)| Bot::new(username, password))
        .collect::<FuturesOrdered<_>>()
        .try_collect::<Vec<_>>()
        .await?;
    let (bots, futures): (Vec<_>, FuturesUnordered<_>) = bots
        .into_iter()
        .enumerate()
        .map(|(i, (bot, future))| {
            let username = bot.username.clone();
            (bot, future.map(move |x| (i, username, x)))
        })
        .unzip();
    Ok((bots, futures))
}
