use super::super::common::Attachment;
use crate::events::common::{CommonMessage, MessageFrom};
use serde::{Deserialize, Serialize};

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

impl CommonMessage for GroupAtMessage {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_content(&self) -> &Option<String> {
        &self.content
    }

    fn get_author_openid(&self) -> &String {
        &self.author.member_openid
    }

    fn get_timestamp(&self) -> &Option<String> {
        &self.timestamp
    }

    fn get_attachments(&self) -> &Option<Vec<Attachment>> {
        &self.attachments
    }

    fn get_msg_seq(&self) -> &Option<u64> {
        &self.msg_seq
    }

    fn get_message_from_type() -> MessageFrom {
        MessageFrom::Group
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
