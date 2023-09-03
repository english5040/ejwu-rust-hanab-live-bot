#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::wildcard_imports)]

mod chat_command;
mod command;
mod hanabi_client;

use std::collections::HashMap;

use clap::Parser;
use color_eyre::eyre::{self, eyre};
use command::client;
use futures::prelude::*;
use futures::stream::FuturesUnordered;
use serde::Deserialize;

use crate::hanabi_client::Bot;

// Args apply to all bots, except create: one bot creates a table and the
// others all join it.
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
    // We currently don't allow specifying bot login passwords

    // Whether to create a table. If table not set, use a random name
    // When create is true, bots will join the first bot's table automatically
    #[arg(short, long, group = "create_or_join")]
    create: bool,
    // Table to create or table to join
    #[arg(short, long, value_name = "NAME", group = "join_target")]
    table: Option<String>,
    // User to join. Named follow_user because we'll try to join whichever table
    // that user goes to.
    #[arg(
        short,
        long,
        value_name = "USERNAME",
        group = "create_or_join",
        group = "join_target"
    )]
    follow_user: Option<String>,
    // Password to use when creating or joining tables. The setting applies to
    // all future tables until changed.
    #[arg(short, long)]
    password: Option<String>,
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

    let bot_usernames = match &args.user {
        Some(users) => users.as_slice(),
        None => &config.default_bots[0..args.n],
    };
    let bot_username_passwords = bot_usernames.iter().map(|username| {
        let username = username.clone();
        let password = config.bots[&username].clone();
        (username, password)
    });
    let mut bot_new_results: FuturesUnordered<_> = bot_username_passwords
        .enumerate()
        .map(|(i, (username, password))| {
            tokio::spawn(
                async move { (i, Bot::new(&username, &password).await) },
            )
        })
        .collect();

    // Collection that holds running bot futures; the bots are spawned by
    // ezsockets when created (kinda bad API) so no need to spawn them
    let mut running_bot_futures = FuturesUnordered::new();
    // Helper function to process args
    // Should impl Fn
    let process_args_for_bot = |i: usize, bot: Bot| {
        if args.create {
            match i {
                0 => bot.create_table(&client::TableCreate {
                    name: args.table.clone(),
                    ..client::TableCreate::default()
                }),
                _ => bot.follow_user(bot_usernames[0].clone()),
            }
        } else if let Some(user) = &args.follow_user {
            bot.follow_user(user.clone());
        } else if let Some(table) = &args.table {
            bot.join_table(table.clone());
        }
    };

    while let Some(join_result) = bot_new_results.next().await {
        // Threads always complete successfully, so unwrap()
        let (i, result) = join_result.unwrap();
        match result {
            Ok((bot, future)) => {
                process_args_for_bot(i, bot);
                running_bot_futures.push(async move { (i, future.await) });
            }
            Err(e) => {
                tracing::error!(
                    "bot[{i}] {{username={:?}}} error when starting: {e}",
                    bot_usernames[i]
                );
            }
        }
    }
    while let Some((i, result)) = running_bot_futures.next().await {
        match result {
            Ok(()) => {
                tracing::info!(
                    "bot[{i}] {{username={:?}}} finished",
                    bot_usernames[i]
                );
            }
            Err(e) => {
                tracing::error!(
                    "bot[{i}] {{username={:?}}} terminated with error: {e}",
                    bot_usernames[i]
                );
            }
        }
    }
    Ok(())
}
