use serde::{Deserialize, Serialize};
use crate::events::common::Message;
use super::super::common::{Attachment};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAuthor {
    pub member_openid: String,
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

impl Message for GroupAtMessage {
    fn get_content(&self) -> Option<String> {
        self.content.clone()
    }

    fn get_author_openid(&self) -> String {
        self.author.member_openid.clone()
    }
}

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
