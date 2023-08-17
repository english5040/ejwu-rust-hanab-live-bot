use std::collections::HashMap;

use async_trait::async_trait;
use color_eyre::eyre::eyre;
use serde::Serialize;

use crate::command::*;

// TODO make private, expose handle only
#[derive(Debug)]
pub struct HanabiClientState {
    handle: ezsockets::Client<HanabiClientState>,
    tables: HashMap<TableID, server::Table>,
    // Current table
    current_table: Option<TableID>,
}

impl HanabiClientState {
    pub fn new(handle: ezsockets::Client<HanabiClientState>) -> Self {
        HanabiClientState {
            handle,
            tables: HashMap::new(),
            current_table: None,
        }
    }
}

#[async_trait]
impl ezsockets::ClientExt for HanabiClientState {
    type Call = fn(&mut Self);

    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        let (name, data) = text.split_once(' ').ok_or_else(|| {
            eyre!("error parsing command: no space in command")
        })?;
        match name {
            <server::Warning>::NAME => {
                let server::Warning { warning } = serde_json::from_str(data)?;
                tracing::info!("received warning from server: {warning}");
            }
            <server::Error>::NAME => {
                let server::Error { error } = serde_json::from_str(data)?;
                return Err(eyre!("received error from server: {error}").into());
            }
            <server::Welcome>::NAME => {
                let welcome: server::Welcome = serde_json::from_str(data)?;
            }
            <server::Name>::NAME => {}
            <server::Table>::NAME => {
                let table: server::Table = serde_json::from_str(data)?;
                self.tables.insert(table.id, table);
            }
            <server::TableList>::NAME => {
                let server::TableList(list) = serde_json::from_str(data)?;
                self.tables
                    .extend(list.into_iter().map(|table| (table.id, table)));
            }
            _ => {
                tracing::info!("received unknown command: {name:?}");
            }
        }
        Ok(())
    }

    async fn on_binary(
        &mut self,
        _bytes: Vec<u8>,
    ) -> Result<(), ezsockets::Error> {
        Err(eyre!("received binary message from server").into())
    }

    async fn on_call(
        &mut self,
        call: Self::Call,
    ) -> Result<(), ezsockets::Error> {
        call(self);
        Ok(())
    }
}

trait SendCommand {
    fn send_command<T>(&self, command: T)
    where
        T: Command + Serialize;
}

impl SendCommand for ezsockets::Client<HanabiClientState> {
    fn send_command<T>(&self, command: T)
    where
        T: Command + Serialize,
    {
        self.text(command.serialize_command());
    }
}

trait HanabiClientHandlePrivate {}

// TODO wrap the Client<HanabiClientState> in a newtype
impl HanabiClientHandlePrivate for ezsockets::Client<HanabiClientState> {}

pub trait HanabiClientHandle {
    // Create and join table
    fn create_table(&self);

    fn join_table(&self, table_id: TableID);

    // Start the current table
    fn start(&self);
}

impl HanabiClientHandle for ezsockets::Client<HanabiClientState> {
    fn create_table(&self) {
        let table = client::TableCreate {
            name: None,
            max_players: 6,
        };
        self.send_command(table);
    }

    fn join_table(&self, table_id: TableID) {
        self.send_command(client::TableJoin { table_id });
    }

    fn start(&self) {
        self.call(|client| {
            let Some(current_table) = client.current_table else {
                return
            };
            client.handle.join_table(current_table)
        });
    }
}
