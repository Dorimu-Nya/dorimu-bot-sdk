use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioOrLiveChannelMemberEvent {
    pub guild_id: String,
    pub channel_id: String,
    #[serde(rename = "type")]
    pub channel_type: Option<i64>,
    pub user_id: String,
}
