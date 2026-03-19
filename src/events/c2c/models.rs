use serde::{Deserialize, Serialize};
use super::super::common::Attachment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2cAuthor {
    pub id: Option<String>,
    pub user_openid: String,
    pub union_openid: Option<String>,
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
pub struct FriendAddEvent {
    pub timestamp: i64,
    pub openid: String,
    pub scene: Option<i64>,
    pub scene_param: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendDelEvent {
    pub timestamp: i64,
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2cMsgRejectEvent {
    pub timestamp: i64,
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2cMsgReceiveEvent {
    pub timestamp: i64,
    pub openid: String,
}
