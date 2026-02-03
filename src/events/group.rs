use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAddRobotEvent {
    pub timestamp: i64,
    pub group_openid: String,
    pub op_member_openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDelRobotEvent {
    pub timestamp: i64,
    pub group_openid: String,
    pub op_member_openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMsgRejectEvent {
    pub timestamp: i64,
    pub group_openid: String,
    pub op_member_openid: String,
}