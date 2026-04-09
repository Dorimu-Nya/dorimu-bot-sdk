use serde::{Deserialize, Serialize};

// 放一些各个事件通用的模型

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub content_type: Option<String>,
    pub filename: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
    pub size: Option<i64>,
    pub url: Option<String>,
}

pub enum MessageFrom {
    C2c,
    Group,
}

// 对c2c和群组的消息的简单封装抽象
pub trait CommonMessage: Sync {
    fn get_id(&self) -> &String;
    fn get_content(&self) -> &Option<String>;
    fn get_author_openid(&self) -> &String;
    fn get_timestamp(&self) -> &Option<String>;
    fn get_attachments(&self) -> &Option<Vec<Attachment>>;
    fn get_msg_seq(&self) -> &Option<u64>;
    fn get_message_from_type() -> MessageFrom
    where
        Self: Sized;
}

/// CommonMessage 的转换 trait， 用于在处理command宏的时候的转换
pub trait FromCommonMessage<'a>: Sized {
    fn from(req: &'a dyn CommonMessage) -> Self;
}

impl<'a> FromCommonMessage<'a> for &'a dyn CommonMessage {
    fn from(req: &'a dyn CommonMessage) -> Self {
        req
    }
}

impl<'a> FromCommonMessage<'a> for &'a Option<Vec<Attachment>> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        req.get_attachments()
    }
}

impl<'a> FromCommonMessage<'a> for Option<Vec<&'a str>> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        match req.get_content() {
            None => None,
            Some(msg) => Some(msg.split_whitespace().collect()),
        }
    }
}

impl<'a> FromCommonMessage<'a> for &'a Option<String> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        &req.get_content()
    }
}
