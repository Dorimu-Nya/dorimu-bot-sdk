use crate::models::message::{MessageMarkdown};
use crate::openapi::models::message::MessageType;
use crate::openapi::models::message::{MessageArk, MessageEmbed, MessageMedia, SendMessageRequest};
use serde::{Deserialize, Serialize};

/// 指示回复的会话类型：私聊（C2c）或群组（Group）。
pub enum ReplyingType {
    /// 一对一私聊。
    C2c,
    /// 群组聊天。
    Group,
}

/// 回复消息的类型
///
/// 变体：
/// - `Text`：纯文本消息
/// - `Markdown`：Markdown 模式消息
/// - `Ark`：Ark 模板消息
/// - `Embed`：嵌入卡片消息
/// - `Media`：媒体消息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyingMessage {
    /// 纯文本消息。
    Text(String),
    /// Markdown 模板消息。
    Markdown(MessageMarkdown),
    /// Ark 模板消息。
    Ark(MessageArk),
    /// 嵌入卡片消息。
    Embed(MessageEmbed),
    /// 媒体消息。
    Media(MessageMedia),
}

impl ReplyingMessage {
    /// 将枚举映射到msg_type的数值。
    pub fn to_msg_type(&self) -> MessageType {
        match self {
            Self::Text(_) => MessageType::Text,
            Self::Markdown(_) => MessageType::Markdown,
            Self::Ark(_) => MessageType::Ark,
            Self::Embed(_) => MessageType::Embed,
            Self::Media(_) => MessageType::Media,
        }
    }

    pub fn to_request(&self, msg_id: Option<String>, msg_seq: Option<u64>) -> SendMessageRequest {
        let basic = SendMessageRequest {
            msg_id,
            msg_seq,
            msg_type: self.to_msg_type().into(),
            ..Default::default()
        };
        match self {
            Self::Text(text) => SendMessageRequest {
                content: Some(text.clone()),
                ..basic
            },
            Self::Markdown(markdown) => SendMessageRequest {
                markdown: Some(markdown.clone()),
                keyboard: match &markdown.keyboard {
                    Some(keyboard) => Some(keyboard.clone()),
                    None => None,
                },
                ..basic
            },
            Self::Ark(ark) => SendMessageRequest {
                ark: Some(ark.clone()),
                ..basic
            },
            Self::Embed(embed) => SendMessageRequest {
                embed: Some(embed.clone()),
                ..basic
            },
            Self::Media(media) => SendMessageRequest {
                media: Some(media.clone()),
                ..basic
            },
        }
    }
}
