use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub content_type: Option<String>,
    pub filename: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
    pub size: Option<i64>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub public_flags: Option<i64>,
    pub system: Option<bool>,
    pub union_openid: Option<String>,
    pub union_user_account: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub joined_at: Option<String>,
    pub roles: Option<Vec<String>>,
    #[serde(default)]
    pub nick: Option<String>,
    #[serde(default)]
    pub deaf: Option<bool>,
    #[serde(default)]
    pub mute: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    pub guild_id: Option<String>,
}