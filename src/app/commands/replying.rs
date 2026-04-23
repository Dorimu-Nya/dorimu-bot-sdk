use serde::{Deserialize, Serialize};

/// Markdown 消息
/// 
/// https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/type/markdown.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingMarkdown {
    /// 本次回复的 Markdown 文本内容（可选）。
    pub content: Option<String>,
    /// 自定义模板 ID（可选）。
    pub custom_template_id: Option<String>,
    /// 模板参数（可选）。
    pub params: Option<Vec<ReplyingMarkdownParam>>,
}

/// Markdown 模板参数项，表示一个键对应多个值。
///
/// - `key`：参数名。
/// - `values`：参数可以有的多个值（按顺序或作为备选）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingMarkdownParam {
    /// 参数名。
    pub key: String,
    /// 参数值列表。
    pub values: Vec<String>,
}

/// ARK 消息
/// 
/// https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/type/ark.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingArk {
    /// Ark 模板 ID。
    pub template_id: u64,
    /// Ark 模板的键值对。
    pub kv: Vec<ReplyingArkKv>,
}

/// Ark 模板中的键值对项。
///
/// - `key`：字段名或占位符名。
/// - `value`：对应的字符串值。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingArkKv {
    /// 字段名。
    pub key: String,
    /// 字段值。
    pub value: String,
}

///  Embed 消息
/// 
/// https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/type/embed.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingEmbed {
    /// 卡片标题（可选）。
    pub title: Option<String>,
    /// 卡片提示或描述（可选）。
    pub prompt: Option<String>,
    /// 缩略图（可选）。
    pub thumbnail: Option<ReplyingEmbedThumbnail>,
    /// 卡片字段（可选）。
    pub fields: Option<Vec<ReplyingEmbedField>>,
}

/// 嵌入卡片的缩略图信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingEmbedThumbnail {
    /// 缩略图 URL（可选）。
    pub url: Option<String>,
}

/// 嵌入卡片中的单个字段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingEmbedField {
    /// 字段名（可选）。
    pub name: Option<String>,
    /// 字段值（可选）。
    pub value: Option<String>,
}

/// 媒体类型的消息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingMedia {
    /// 文件类型（数字编码）。
    pub file_type: u8,
    /// 文件 URL。
    pub url: String,
    /// 是否由服务端发送消息。
    pub srv_send_msg: bool,
    /// 可选的文件数据（例如 base64 字符串）。
    pub file_data: Option<String>,
}

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
    Markdown(ReplyingMarkdown),
    /// Ark 模板消息。
    Ark(ReplyingArk),
    /// 嵌入卡片消息。
    Embed(ReplyingEmbed),
    /// 媒体消息。
    Media(ReplyingMedia),
}

impl ReplyingMessage {
    /// 将枚举映射到msg_type的数值。
    pub fn to_msg_type(&self) -> u8 {
        match self {
            Self::Text(_) => 0,
            Self::Markdown(_) => 2,
            Self::Ark(_) => 3,
            Self::Embed(_) => 4,
            Self::Media(_) => 7,
        }
    }
}
