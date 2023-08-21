use std::num::NonZeroU64;

use color_eyre::eyre::{self, eyre};
use serde::{Deserialize, Serialize};

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

pub enum Parse<'a, O> {
    Break(eyre::Result<O>),
    Continue(&'a str, &'a str),
}

impl<'a, O> Parse<'a, O> {
    pub fn from_str(s: &'a str) -> Self {
        match s.split_once(' ') {
            Some((name, data)) => Self::Continue(name, data),
            None => Self::Break(Err(eyre!("error parsing command: no space"))),
        }
    }
    pub fn handle_command<T, F>(self, f: F) -> Self
    where
        T: Command + Deserialize<'a>,
        F: FnOnce(T) -> O,
    {
        self.handle_command_result(|x| Ok(f(x)))
    }
    pub fn handle_command_result<T, F>(self, f: F) -> Self
    where
        T: Command + Deserialize<'a>,
        F: FnOnce(T) -> eyre::Result<O>,
    {
        match self {
            Self::Continue(name, data) if name == T::NAME => {
                let result = match serde_json::from_str(data) {
                    Ok(command) => f(command),
                    Err(e) => Err(e.into()),
                };
                Self::Break(result)
            }
            _ => self,
        }
    }
    pub fn unhandled<F>(self, f: F) -> eyre::Result<O>
    where
        F: FnOnce(&'a str, &'a str) -> eyre::Result<O>,
    {
        match self {
            Self::Break(r) => r,
            Self::Continue(name, data) => f(name, data),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserID(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TableID(u64);
// TODO use NonZeroU64
