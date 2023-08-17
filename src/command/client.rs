use serde::Serialize;
use serde_with::skip_serializing_none;

use super::{Command, TableID, UserID};

// --- Lobby

// Current Table

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCreate {
    pub name: Option<String>,
    pub max_players: i32,
}
impl Default for TableCreate {
    fn default() -> Self {
        TableCreate {
            name: Default::default(),
            max_players: 6,
        }
    }
}
impl Command for TableCreate {
    const NAME: &'static str = "tableCreate";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableJoin {
    #[serde(rename = "tableID")]
    pub table_id: TableID,
}
impl Command for TableJoin {
    const NAME: &'static str = "tableJoin";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableLeave {
    #[serde(rename = "tableID")]
    pub table_id: TableID,
}
impl Command for TableLeave {
    const NAME: &'static str = "tableLeave";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableUnattend {
    #[serde(rename = "tableID")]
    pub table_id: TableID,
}
impl Command for TableUnattend {
    const NAME: &'static str = "tableUnattend";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableReattend {
    #[serde(rename = "tableID")]
    pub table_id: TableID,
}
impl Command for TableReattend {
    const NAME: &'static str = "tableReattend";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableSetVariant {}
impl Command for TableSetVariant {
    const NAME: &'static str = "tableSetVariant";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableSetLeader {}
impl Command for TableSetLeader {
    const NAME: &'static str = "tableSetLeader";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableStart {
    #[serde(rename = "tableID")]
    pub table_id: TableID,
    // TODO necessary?
    // replay: bool,
}
impl Command for TableStart {
    const NAME: &'static str = "tableStart";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableVoteForTermination {}
impl Command for TableVoteForTermination {
    const NAME: &'static str = "tableVoteForTermination";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableTerminate {}
impl Command for TableTerminate {
    const NAME: &'static str = "tableTerminate";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRestart {}
impl Command for TableRestart {
    const NAME: &'static str = "tableRestart";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableUpdate {}
impl Command for TableUpdate {
    const NAME: &'static str = "tableUpdate";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableSuggest {}
impl Command for TableSuggest {
    const NAME: &'static str = "tableSuggest";
}

// Chat

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {}
impl Command for Chat {
    const NAME: &'static str = "chat";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatPM {}
impl Command for ChatPM {
    const NAME: &'static str = "chatPM";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatRead {}
impl Command for ChatRead {
    const NAME: &'static str = "chatRead";
}

// Other commands

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetName;
impl Command for GetName {
    const NAME: &'static str = "getName";
}

// --- In game

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {}
impl Command for Action {
    const NAME: &'static str = "action";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {}
impl Command for Note {
    const NAME: &'static str = "note";
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pause {}
impl Command for Pause {
    const NAME: &'static str = "pause";
}
