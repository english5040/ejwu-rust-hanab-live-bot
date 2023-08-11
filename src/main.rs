mod client_command;
mod deserialize_space_separated_command;
mod hanabi_client;
mod serialize_command;
mod server_command;

use std::sync::Arc;

use anyhow::{anyhow, Context};
use reqwest::cookie::CookieStore;
use serde_json::json;
use tracing::instrument;
use url::Url;

use crate::hanabi_client::HanabiClient;

#[instrument]
async fn authenticate_and_get_cookie() -> anyhow::Result<http::HeaderValue> {
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
            "username": "ejwu-bot1",
            "password": "ejwu-bot1",
            "version": "bot",
        }))
        .send()
        .await?;
    response.error_for_status_ref().with_context(|| {
        format!("Authentication failed. Server response: {response:#?}")
    })?;
    let cookie = jar
        .cookies(&url)
        .context("No cookie was received from hanab.live server")?;
    Ok(cookie)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let cookie = authenticate_and_get_cookie().await?;

    // Run the client
    let config = ezsockets::ClientConfig::new("wss://hanab.live/ws")
        .header(http::header::COOKIE, cookie);
    // TODO ezsockets is a really small hobby package.
    // Maybe use a different websocket client library.
    // Maybe pull directly from Github so new fixes are brought in immediately.
    let (_handle, future) =
        ezsockets::connect(|_handle| HanabiClient {}, config).await;
    future.await.map_err(|e| anyhow!(e))?;
    Ok(())
}
