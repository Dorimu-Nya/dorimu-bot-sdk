use super::super::common::Attachment;
use crate::events::common::{CommonMessage, MessageFrom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 单聊消息作者
pub struct C2cAuthor {
    /// 作者id
    pub id: Option<String>,
    /// 用户openid
    pub user_openid: String,
    /// 联合openid
    pub union_openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 消息场景
pub struct C2cMessageScene {
    /// 来源
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 单聊消息
///
/// 触发场景：用户在单聊发送消息给机器人
pub struct C2cMessage {
    /// 平台方消息ID，可以用于被动消息发送
    pub id: String,
    /// 发送者
    pub author: C2cAuthor,
    /// 文本消息内容
    pub content: Option<String>,
    /// 消息生产时间（RFC3339）
    pub timestamp: Option<String>,
    /// 富媒体文件附件，文件类型："图片，语音，视频，文件"
    pub attachments: Option<Vec<Attachment>>,
    /// 消息类型
    pub message_type: Option<u32>,
    /// 场景信息
    pub message_scene: Option<C2cMessageScene>,
    #[serde(default)]
    /// 消息序列
    pub msg_seq: Option<u64>,
}

impl CommonMessage for C2cMessage {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_content(&self) -> &Option<String> {
        &self.content
    }

    fn get_author_openid(&self) -> &String {
        &self.author.user_openid
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
        MessageFrom::C2c
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 用户添加机器人
pub struct FriendAddEvent {
    /// 添加时间戳
    pub timestamp: i64,
    /// 用户openid
    pub openid: String,
    /// 场景 id
    pub scene: Option<i64>,
    /// 场景参数
    pub scene_param: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 用户删除机器人
pub struct FriendDelEvent {
    /// 删除时间戳
    pub timestamp: i64,
    /// 用户openid
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 拒绝机器人主动消息
pub struct C2cMsgRejectEvent {
    /// 操作时间戳
    pub timestamp: i64,
    /// 用户openid
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 允许机器人主动消息
pub struct C2cMsgReceiveEvent {
    /// 操作时间戳
    pub timestamp: i64,
    /// 用户openid
    pub openid: String,
}
