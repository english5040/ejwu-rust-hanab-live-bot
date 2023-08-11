use serde::{Deserialize, Serialize};

pub use super::{TableID, UserID};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ServerCommand {
    // Server messages
    Warning { warning: String },
    Error { error: String },

    // --- Lobby
    // General information
    Welcome(WelcomeData),
    Name { name: String },
    // Tables
    Table(TableData),
    TableList(Vec<TableData>),
    TableStart(SingleTableID),
    TableProgress(SingleTableID),
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
    // Actions on tables
    Joined,
    Left,

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
pub struct SingleUserID {
    #[serde(rename = "userID")]
    user_id: UserID,
}

#[derive(Debug, Deserialize)]
pub struct SingleTableID {
    #[serde(rename = "tableID")]
    table_id: TableID,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeData {
    #[serde(rename = "userID")]
    user_id: UserID,
    random_table_name: String,
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
