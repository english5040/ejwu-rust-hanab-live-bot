mod command;
mod hanabi_client;

use std::{collections::HashMap, error::Error, sync::Arc};

use clap::Parser;
use color_eyre::eyre::{self, eyre, WrapErr};
use futures::{Future, FutureExt};
use reqwest::cookie::CookieStore;
use serde::Deserialize;
use serde_json::json;
use tracing::instrument;
use url::Url;

use crate::hanabi_client::{HanabiClientHandle, HanabiClientState};

#[derive(clap::Parser)]
struct Args {
    // Number of bots to run
    #[arg(short, default_value_t = 1)]
    n: i32,
    // Usernames
    #[arg(long)]
    user: Option<Vec<String>>,
    // Whether to create a table. If table not set, use a random name
    #[arg(long)]
    create: bool,
    // Table to join
    #[arg(long)]
    table: Option<String>,
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

    let f = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&f)?;

    let args = Args::parse();

    // Make bots (just 1 and sequentially for now)
    let username = &config.default_bots[0];
    let (handle, future) = make_bot(username, &config.bots[username]).await?;
    // Run bot
    let join_handle = tokio::spawn(future);
    // Handle arguments
    if args.create {
        handle.create_table();
    }
    join_handle.await??;
    Ok(())
}

async fn make_bot(
    username: &str,
    password: &str,
) -> eyre::Result<(
    ezsockets::Client<HanabiClientState>,
    impl Future<Output = eyre::Result<()>>,
)> {
    let cookie = authenticate_and_get_cookie(username, password).await?;

    let config = ezsockets::ClientConfig::new("wss://hanab.live/ws")
        .header(http::header::COOKIE, cookie);
    // TODO ezsockets is a really small hobby crate.
    // Maybe use a different websocket client library.
    // Maybe pull directly from Github so new fixes are brought in immediately.
    let (handle, future) =
        ezsockets::connect(HanabiClientState::new, config).await;
    let future = future.map(|result| result.map_err(|e| eyre!(e)));
    Ok((handle, future))
}

#[instrument]
async fn authenticate_and_get_cookie(
    username: &str,
    password: &str,
) -> eyre::Result<http::HeaderValue> {
    // Authenticate to hanab.live, grab cookie from response
    let url = Url::parse("https://hanab.live/login")?;

    // Temporary client
    let jar = Arc::new(reqwest::cookie::Jar::default());
    let client = reqwest::ClientBuilder::new()
        .cookie_provider(jar.clone())
        .build()?;
    let response = client
        .post(url.clone())
        .form(&json!({
            "username": username,
            "password": password,
            "version": "bot",
        }))
        .send()
        .await?;
    response.error_for_status_ref().wrap_err_with(|| {
        format!("Authentication failed. Server response: {response:#?}")
    })?;
    let cookie = jar.cookies(&url).ok_or_else(|| {
        eyre!("No cookie was received from hanab.live server")
    })?;
    Ok(cookie)
}
