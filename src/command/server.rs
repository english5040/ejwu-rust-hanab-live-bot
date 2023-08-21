use serde::Deserialize;

use super::*;

// Server messages

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Warning {
    pub warning: String,
}
impl Command for Warning {
    const NAME: &'static str = "warning";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub error: String,
}
impl Command for Error {
    const NAME: &'static str = "error";
}

// --- Lobby

// General information

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Welcome {
    #[serde(rename = "userID")]
    pub user_id: UserID,
    pub random_table_name: String,
}
impl Command for Welcome {
    const NAME: &'static str = "welcome";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub name: String,
}
impl Command for Name {
    const NAME: &'static str = "name";
}

// Tables

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub id: TableID,
    pub name: String,
}
impl Command for Table {
    const NAME: &'static str = "table";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableList(pub Vec<Table>);
impl Command for TableList {
    const NAME: &'static str = "tableList";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableStart {
    #[serde(rename = "tableID")]
    pub table_id: TableID,
}
impl Command for TableStart {
    const NAME: &'static str = "tableStart";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableProgress {
    #[serde(rename = "tableID")]
    pub table_id: TableID,
}
impl Command for TableProgress {
    const NAME: &'static str = "tableProgress";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableGone {
    #[serde(rename = "tableID")]
    pub table_id: TableID,
}
impl Command for TableGone {
    const NAME: &'static str = "tableGone";
}

// Users

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "userID")]
    pub user_id: UserID,
    pub name: String,
    // TODO make this an enum
    pub status: i32,
    // TODO 0 is used as no TableID, so convert to Option<TableID>
    #[serde(rename = "tableID")]
    pub table_id: TableID,
}
impl Command for User {
    const NAME: &'static str = "user";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserList(pub Vec<User>);
impl Command for UserList {
    const NAME: &'static str = "userList";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLeft {
    #[serde(rename = "userID")]
    pub user_id: UserID,
}
impl Command for UserLeft {
    const NAME: &'static str = "userLeft";
}

// Games

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameHistory;
impl Command for GameHistory {
    const NAME: &'static str = "gameHistory";
}

// Chat

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub recipient: String,
    pub who: String,
    pub msg: String,
}
impl Command for Chat {
    const NAME: &'static str = "chat";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatList {
    pub list: Vec<Chat>,
}
impl Command for ChatList {
    const NAME: &'static str = "chatList";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatTyping;
impl Command for ChatTyping {
    const NAME: &'static str = "chatTyping";
}

// Actions on tables

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Joined;
impl Command for Joined {
    const NAME: &'static str = "joined";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Left;
impl Command for Left {
    const NAME: &'static str = "left";
}

// --- In game

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Init;
impl Command for Init {
    const NAME: &'static str = "init";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameAction;
impl Command for GameAction {
    const NAME: &'static str = "gameAction";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameActionList;
impl Command for GameActionList {
    const NAME: &'static str = "gameActionList";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseID;
impl Command for DatabaseID {
    const NAME: &'static str = "databaseID";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connected;
impl Command for Connected {
    const NAME: &'static str = "connected";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clock;
impl Command for Clock {
    const NAME: &'static str = "clock";
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteListPlayer;
impl Command for NoteListPlayer {
    const NAME: &'static str = "noteListPlayer";
}
