use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod client;
pub mod server;

pub trait Command {
    const NAME: &'static str;

    fn serialize_command(&self) -> String
    where
        Self: Serialize,
    {
        let mut s = Self::NAME.to_owned();
        s.push(' ');
        s.push_str(&serde_json::to_string(&self).unwrap());
        s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserID(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TableID(i32);
