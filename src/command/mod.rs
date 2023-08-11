use serde::{Deserialize, Serialize};

mod client;
mod deserialize;
mod serialize;
mod server;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserID(i32);

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TableID(i32);

pub use client::*;
pub use deserialize::*;
pub use serialize::*;
pub use server::*;
