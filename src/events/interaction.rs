use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionResolved {
    pub button_data: Option<String>,
    pub button_id: Option<String>,
    pub user_id: Option<String>,
    pub feature_id: Option<String>,
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionData {
    #[serde(rename = "type")]
    pub kind: Option<i64>,
    #[serde(default, alias = "resoloved")]
    pub resolved: Option<InteractionResolved>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: Option<i64>,
    pub scene: Option<String>,
    pub chat_type: Option<i64>,
    pub timestamp: Option<String>,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub user_openid: Option<String>,
    pub group_openid: Option<String>,
    pub group_member_openid: Option<String>,
    pub data: Option<InteractionData>,
    pub version: Option<i64>,
}