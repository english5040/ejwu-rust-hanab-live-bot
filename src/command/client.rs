use serde::Serialize;

use super::{TableID, UserID};

// --- Lobby

// Current Table

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCreate {
    #[serde(rename = "tableID")]
    table_id: TableID,
}
#[derive(Debug, Serialize)]
pub struct TableJoin {
    #[serde(rename = "tableID")]
    table_id: TableID,
}
#[derive(Debug, Serialize)]
pub struct TableLeave {}
#[derive(Debug, Serialize)]
pub struct TableUnattend {}
#[derive(Debug, Serialize)]
pub struct TableReattend {}
#[derive(Debug, Serialize)]
pub struct TableSetVariant {}
#[derive(Debug, Serialize)]
pub struct TableSetLeader {}
#[derive(Debug, Serialize)]
pub struct TableStart {}
#[derive(Debug, Serialize)]
pub struct TableVoteForTermination {}
#[derive(Debug, Serialize)]
pub struct TableTerminate {}
#[derive(Debug, Serialize)]
pub struct TableRestart {}
#[derive(Debug, Serialize)]
pub struct TableUpdate {}
#[derive(Debug, Serialize)]
pub struct TableSuggest {}

// Chat

#[derive(Debug, Serialize)]
pub struct Chat {}
#[derive(Debug, Serialize)]
pub struct ChatPM {}
#[derive(Debug, Serialize)]
pub struct ChatRead {}
#[derive(Debug, Serialize)]

// Other commands

pub struct GetName {}
#[derive(Debug, Serialize)]

// --- In game

pub struct Action {}
#[derive(Debug, Serialize)]
pub struct Note {}
#[derive(Debug, Serialize)]
pub struct Pause {}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientCommand {
    TableCreate(TableCreate),
    TableJoin(TableJoin),
    TableLeave(TableLeave),
    TableUnattend(TableUnattend),
    TableReattend(TableReattend),
    TableSetVariant(TableSetVariant),
    TableSetLeader(TableSetLeader),
    TableStart(TableStart),
    TableVoteForTermination(TableVoteForTermination),
    TableTerminate(TableTerminate),
    TableRestart(TableRestart),
    TableUpdate(TableUpdate),
    TableSuggest(TableSuggest),
    Chat(Chat),
    ChatPM(ChatPM),
    ChatRead(ChatRead),
    GetName(GetName),
    Action(Action),
    Note(Note),
    Pause(Pause),
}
