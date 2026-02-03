use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildEvent {
    pub id: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub op_user_id: Option<String>,
    pub joined_at: Option<String>,
    pub description: Option<String>,
    pub member_count: Option<i64>,
    pub max_members: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelEvent {
    pub guild_id: String,
    pub id: String,
    pub name: Option<String>,
    pub owner_id: Option<String>,
    pub op_user_id: Option<String>,
    pub sub_type: Option<i64>,
    #[serde(rename = "type")]
    pub channel_type: Option<i64>,
}