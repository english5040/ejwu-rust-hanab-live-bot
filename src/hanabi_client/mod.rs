use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use color_eyre::eyre::{self, eyre, WrapErr};
use futures::prelude::*;
use reqwest::cookie::CookieStore;
use serde::Serialize;
use serde_json::json;
use tracing::instrument;
use url::Url;

use crate::command::{self, client, server, Command, TableID, UserID};

#[derive(Debug)]
struct State {
    // --- Wrapped ezsockets::Client handle
    handle: Bot,
    // --- State for hanab.live lobby
    users: HashMap<UserID, server::User>,
    tables: HashMap<TableID, server::Table>,
    // Current table
    // TODO if we are currently creating a table, this might be delayed
    current_table: Option<TableID>,
    // --- Bot-specific state for lobby
    // Constantly try to go to this user's table.
    following_user: Option<String>,
    // Table we want to join. Only join the table once, though.
    join_table: Option<String>,
}

impl State {
    fn new(username: String, handle: ezsockets::Client<Self>) -> Self {
        Self {
            handle: Bot::from_handle(username, handle),
            users: HashMap::new(),
            tables: HashMap::new(),
            current_table: None,
            following_user: None,
            join_table: None,
        }
    }
    fn username(&self) -> &str {
        &self.handle.username
    }
    fn insert_user(&mut self, user: server::User) {
        self.users.insert(user.user_id, user);
    }
    fn remove_user(&mut self, user_id: UserID) {
        let removed = self.users.remove(&user_id);
        if removed.is_none() {
            tracing::error!("called remove_user, but {user_id:?} not found");
        }
    }
    fn insert_table(&mut self, table: server::Table) {
        if let Some(join_table) = &self.join_table {
            if &table.name == join_table {
                self.handle
                    .send_command(&client::TableJoin { table_id: table.id });
                self.join_table = None;
            }
        }
        self.tables.insert(table.id, table);
    }
    fn remove_table(&mut self, table_id: TableID) {
        self.tables.remove(&table_id);
    }

    fn join_table(&mut self, table_name: String) {
        for table in self.tables.values() {
            if table.name == table_name {
                self.handle
                    .send_command(&client::TableJoin { table_id: table.id });
                return;
            }
        }
        self.join_table = Some(table_name);
    }
    fn follow_user(&mut self, username: String) {
        self.following_user = Some(username);
        todo!();
    }
    fn start(&mut self) {
        if let Some(current_table) = self.current_table {
            self.handle.send_command(&client::TableStart {
                table_id: current_table,
            });
        }
    }
}

#[derive(Debug)]
enum Call {
    JoinTable(String),
    FollowUser(String),
    Start,
}

impl State {
    fn call(&mut self, call: Call) {
        match call {
            Call::JoinTable(s) => self.join_table(s),
            Call::FollowUser(s) => self.follow_user(s),
            Call::Start => self.start(),
        }
    }
}

#[async_trait]
impl ezsockets::ClientExt for State {
    type Call = Call;

    #[instrument(skip_all, fields(username = self.username()))]
    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        command::Parse::from_str(&text)
            .handle_command(|server::Warning { warning }| {
                tracing::warn!("received warning from server: {warning:?}");
            })
            .handle_command_result(|server::Error { error }| {
                Err(eyre!("received error from server: {error}"))
            })
            .handle_command(|welcome: server::Welcome| {
                tracing::info!("received welcome from server: {welcome:#?}");
            })
            .handle_command(|_: server::Name| {
                // ignored
            })
            .handle_command(|user: server::User| {
                self.insert_user(user);
            })
            .handle_command(|server::UserList(user_list)| {
                for user in user_list {
                    self.insert_user(user);
                }
            })
            .handle_command(|server::UserLeft { user_id }| {
                self.remove_user(user_id);
            })
            .handle_command(|table: server::Table| {
                self.insert_table(table);
            })
            .handle_command(|server::TableList(list)| {
                for table in list {
                    self.insert_table(table);
                }
            })
            .handle_command(|server::TableGone { table_id }| {
                self.remove_table(table_id);
            })
            .unhandled(|name, _data| {
                tracing::info!("received unhandled command {name:?}");
                Ok(())
            })
            .map_err(Into::into)
    }

    async fn on_binary(
        &mut self,
        _bytes: Vec<u8>,
    ) -> Result<(), ezsockets::Error> {
        Err(eyre!("received binary message from server").into())
    }

    #[instrument(skip_all, fields(username = self.username()))]
    async fn on_call(
        &mut self,
        call: Self::Call,
    ) -> Result<(), ezsockets::Error> {
        self.call(call);
        Ok(())
    }
}

mod bot {
    use super::*;

    // Wraps the ezsockets::Client, simplifying the available functionality
    #[derive(Debug)]
    pub struct Bot {
        // Informational
        pub username: String,
        inner: ezsockets::Client<State>,
    }

    impl Bot {
        #[allow(clippy::missing_const_for_fn)]
        pub(super) fn from_handle(
            username: String,
            handle: ezsockets::Client<State>,
        ) -> Self {
            Self {
                username: username.to_owned(),
                inner: handle,
            }
        }
        pub(super) fn send_command<T>(&self, command: &T)
        where
            T: Command + Serialize,
        {
            self.inner.text(command.serialize_command());
        }
        pub(super) fn call(
            &self,
            message: <State as ezsockets::ClientExt>::Call,
        ) {
            self.inner.call(message);
        }
    }
}

pub use bot::Bot;

impl Bot {
    // Construct a Bot. It runs as a spawned task.
    // Returns (bot, future)
    // where future is a JoinHandle for the task running the bot.
    #[instrument(skip(password))]
    pub async fn new(
        username: &str,
        password: &str,
    ) -> eyre::Result<(Self, impl Future<Output = eyre::Result<()>>)> {
        let cookie = authenticate_and_get_cookie(username, password).await?;

        let config = ezsockets::ClientConfig::new("wss://hanab.live/ws")
            .header(http::header::COOKIE, cookie);
        // TODO ezsockets is a really small hobby crate.
        // Maybe use a different websocket client library.
        // Maybe pull directly from Github so new fixes are brought in
        // immediately.
        let username1 = username.to_owned();
        let (handle, future) =
            ezsockets::connect(|handle| State::new(username1, handle), config)
                .await;
        // No need for spawn, ezsockets already spawns
        let future = future.map_err(|e| eyre!(e));
        Ok((Self::from_handle(username.to_owned(), handle), future))
    }

    // Create table. The server will automatically join this bot to the created table
    pub fn create_table(&self, table: &client::TableCreate) {
        self.send_command(table);
    }

    pub fn join_table(&self, table_name: String) {
        self.call(Call::JoinTable(table_name));
    }

    // Start the current table
    pub fn start(&self) {
        self.call(Call::Start);
    }
}

// Authenticate to hanab.live, grab cookie from response
async fn authenticate_and_get_cookie(
    username: &str,
    password: &str,
) -> eyre::Result<http::HeaderValue> {
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
