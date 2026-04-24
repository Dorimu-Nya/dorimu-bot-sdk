use serde::{Deserialize, Serialize};

// 放一些各个事件通用的模型

/// 附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub content_type: Option<String>,
    pub filename: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
    pub size: Option<i64>,
    pub url: Option<String>,
}

/// 消息来源，用于统一消息抽象
pub enum MessageFrom {
    C2c,
    Group,
}

/// 对c2c和群组的消息的简单封装抽象
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

/// CommonMessage 的提取转换 trait， 用于在处理 command宏 的时候的转换
pub trait FromCommonMessage<'a>: Sized {
    fn from(req: &'a dyn CommonMessage) -> Self;
}

/// 自身实现
impl<'a> FromCommonMessage<'a> for &'a dyn CommonMessage {
    fn from(req: &'a dyn CommonMessage) -> Self {
        req
    }
}

/// 提取附件
impl<'a> FromCommonMessage<'a> for &'a Option<Vec<Attachment>> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        req.get_attachments()
    }
}

/// 提取附件（拥有所有权版本）
impl<'a> FromCommonMessage<'a> for Option<Vec<Attachment>> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        req.get_attachments().clone()
    }
}

/// 提取消息
impl<'a> FromCommonMessage<'a> for Option<Vec<&'a str>> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        match req.get_content() {
            None => None,
            Some(msg) => Some(msg.split_whitespace().collect()),
        }
    }
}

/// 提取消息（拥有所有权版本，按空格拆分）
impl<'a> FromCommonMessage<'a> for Option<Vec<String>> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        match req.get_content() {
            None => None,
            Some(msg) => Some(msg.split_whitespace().map(|x| x.to_string()).collect()),
        }
    }
}

/// 提取消息
impl<'a> FromCommonMessage<'a> for &'a Option<String> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        &req.get_content()
    }
}

/// 提取消息（拥有所有权版本）
impl<'a> FromCommonMessage<'a> for Option<String> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        req.get_content().clone()
    }
}

/// 提取消息文本（缺失时返回空字符串）
impl<'a> FromCommonMessage<'a> for String {
    fn from(req: &'a dyn CommonMessage) -> Self {
        req.get_content().clone().unwrap_or_default()
    }
}
