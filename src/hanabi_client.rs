use anyhow::{anyhow, Context};
use async_trait::async_trait;

use crate::server_command::*;

#[derive(Debug)]
pub struct HanabiClient {}

#[async_trait]
impl ezsockets::ClientExt for HanabiClient {
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        let server_command: Result<ServerCommand, _> =
            deserialize_space_separated_command_from_str(&text);
        match server_command {
            Ok(server_command) => {
                tracing::info!("received command: {server_command:?}");
            }
            Err(e) => {
                tracing::error!("error {e} when parsing message: {text}");
            }
        }
        Ok(())
    }

    async fn on_binary(
        &mut self,
        _bytes: Vec<u8>,
    ) -> Result<(), ezsockets::Error> {
        Err(anyhow!("Received binary message from hanab.live server. This should never happen"))?
    }

    async fn on_call(
        &mut self,
        _call: Self::Call,
    ) -> Result<(), ezsockets::Error> {
        Ok(())
    }
}
