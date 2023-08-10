mod deserialize_space_separated_command;

use serde::Deserialize;

pub use deserialize_space_separated_command::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ServerCommand {
    // Server messages
    Warning(serde_json::Value),
    Error(serde_json::Value),

    // --- Lobby
    Welcome(WelcomeData),
    // Tables
    Table(TableData),
    TableList(Vec<TableData>),
    TableGone(SingleTableID),
    // Users
    User(UserData),
    UserList(Vec<UserData>),
    UserLeft(SingleUserID),
    // Games
    GameHistory,
    // Chat
    Chat(ChatData),
    ChatList,
    /* ChatList { list: Vec<ChatData> }, */
    ChatTyping,
    // Actions
    TableStart(SingleTableID),

    // --- In game
    Init,
    GameAction,
    GameActionList,
    DatabaseID,
    Connected,
    Clock,
    NoteListPlayer,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct UserID(i32);

#[derive(Debug, Deserialize)]
pub struct SingleUserID {
    #[serde(rename = "userID")]
    user_id: UserID,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TableID(i32);

#[derive(Debug, Deserialize)]
pub struct SingleTableID {
    #[serde(rename = "tableID")]
    table_id: TableID,
}

#[derive(Debug, Deserialize)]
pub struct WelcomeData {
    #[serde(rename = "userID")]
    user_id: UserID,
}

#[derive(Debug, Deserialize)]
pub struct ChatData {
    recipient: String,
    who: String,
    msg: String,
}

#[derive(Debug, Deserialize)]
pub struct TableData {
    id: TableID,
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    #[serde(rename = "userID")]
    user_id: UserID,
}
