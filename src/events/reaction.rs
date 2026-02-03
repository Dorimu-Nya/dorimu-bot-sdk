use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emoji {
    pub id: String,
    #[serde(rename = "type")]
    pub emoji_type: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionTarget {
    #[serde(rename = "type")]
    pub target_type: Option<i64>,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReaction {
    pub user_id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub target: ReactionTarget,
    pub emoji: Emoji,
}