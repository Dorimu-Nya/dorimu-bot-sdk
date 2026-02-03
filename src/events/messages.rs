use serde::{Deserialize, Serialize};

use super::common::{Attachment, Member, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2cAuthor {
    pub id: Option<String>,
    pub user_openid: String,
    pub union_openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAuthor {
    pub member_openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2cMessageScene {
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2cMessage {
    pub id: String,
    pub author: C2cAuthor,
    pub content: Option<String>,
    pub timestamp: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub message_type: Option<u32>,
    pub message_scene: Option<C2cMessageScene>,
    #[serde(default)]
    pub msg_seq: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAtMessage {
    pub id: String,
    pub author: GroupAuthor,
    pub content: Option<String>,
    pub timestamp: Option<String>,
    pub group_openid: String,
    pub attachments: Option<Vec<Attachment>>,
    #[serde(default)]
    pub msg_seq: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel_id: Option<String>,
    pub guild_id: Option<String>,
    pub content: Option<String>,
    pub timestamp: Option<String>,
    pub author: Option<User>,
    pub member: Option<Member>,
    pub attachments: Option<Vec<Attachment>>,
    pub seq: Option<u64>,
    pub seq_in_channel: Option<u64>,
}
