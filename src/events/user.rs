use serde::{Deserialize, Serialize};

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