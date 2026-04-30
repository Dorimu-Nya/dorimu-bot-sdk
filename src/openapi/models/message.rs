use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 消息类型：0 是文本，2 是 markdown，3 ark，4 embed，7 media 富媒体。
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    /// 文本。
    Text = 0,
    /// Markdown。
    Markdown = 2,
    /// Ark。
    Ark = 3,
    /// Embed。
    Embed = 4,
    /// 富媒体。
    Media = 7,
}

/// 任意 JSON 对象。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct JsonObject(pub BTreeMap<String, Value>);

/// POST /v2/users/{openid}/messages 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequest {
    /// 文本内容。
    #[serde(default)]
    pub content: Option<String>,
    /// 消息类型：0 是文本，2 是 markdown， 3 ark，4 embed，7 media 富媒体。
    pub msg_type: MessageType,
    /// [Markdown](../type/markdown.md#数据结构与协议)对象。
    #[serde(default)]
    pub markdown: Option<MessageMarkdown>,
    /// [Keyboard](../interaction/msg-btn.md#数据结构与协议)对象。
    #[serde(default)]
    pub keyboard: Option<JsonObject>,
    /// [Ark](../type/ark.md#数据结构与协议)对象。
    #[serde(default)]
    pub ark: Option<MessageArk>,
    /// [富媒体单聊](./rich-media.md#用于单聊)的file_info。
    #[serde(default)]
    pub media: Option<MessageMedia>,
    /// Embed消息
    #[serde(default)]
    pub embed: Option<MessageEmbed>,
    /// 【暂未支持】消息引用。
    #[serde(default)]
    pub message_reference: Option<JsonObject>,
    /// 前置收到的事件 ID，用于发送被动消息，支持事件："INTERACTION_CREATE"、"C2C_MSG_RECEIVE"、"FRIEND_ADD"。
    #[serde(default)]
    pub event_id: Option<String>,
    /// 前置收到的用户发送过来的消息 ID，用于发送被动（回复）消息。
    #[serde(default)]
    pub msg_id: Option<String>,
    /// 回复消息的序号，与 msg_id 联合使用，避免相同消息id回复重复发送，不填默认是1。相同的 msg_id + msg_seq 重复发送会失败。
    #[serde(default)]
    pub msg_seq: Option<u64>,
}

impl Default for SendMessageRequest {
    fn default() -> Self {
        Self {
            msg_type: MessageType::Text,
            content: None,
            markdown: None,
            keyboard: None,
            ark: None,
            media: None,
            embed: None,
            message_reference: None,
            event_id: None,
            msg_id: None,
            msg_seq: None,
        }
    }
}

/// POST /v2/users/{openid}/messages 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    /// 消息唯一ID。
    pub id: String,
    /// 发送时间。
    pub timestamp: i64,
}

/// Markdown 消息
///
/// https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/type/markdown.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMarkdown {
    /// 本次回复的 Markdown 文本内容（可选）。
    pub content: Option<String>,
    /// 自定义模板 ID（可选）。
    pub custom_template_id: Option<String>,
    /// 模板参数（可选）。
    pub params: Option<Vec<MessageMarkdownParam>>,
}

/// Markdown 模板参数项，表示一个键对应多个值。
///
/// - `key`：参数名。
/// - `values`：参数可以有的多个值（按顺序或作为备选）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMarkdownParam {
    /// 参数名。
    pub key: String,
    /// 参数值列表。
    pub values: Vec<String>,
}

/// ARK 消息
///
/// https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/type/ark.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArk {
    /// Ark 模板 ID。
    pub template_id: u64,
    /// Ark 模板的键值对。
    pub kv: Vec<MessageArkKv>,
}

/// Ark 模板中的键值对项。
///
/// - `key`：字段名或占位符名。
/// - `value`：对应的字符串值。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArkKv {
    /// 字段名。
    pub key: String,
    /// 字段值。
    pub value: String,
}

///  Embed 消息
///
/// https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/type/embed.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbed {
    /// 卡片标题（可选）。
    pub title: Option<String>,
    /// 卡片提示或描述（可选）。
    pub prompt: Option<String>,
    /// 缩略图（可选）。
    pub thumbnail: Option<MessageEmbedThumbnail>,
    /// 卡片字段（可选）。
    pub fields: Option<Vec<MessageEmbedField>>,
}

/// 嵌入卡片的缩略图信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbedThumbnail {
    /// 缩略图 URL（可选）。
    pub url: Option<String>,
}

/// 嵌入卡片中的单个字段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbedField {
    /// 字段名（可选）。
    pub name: Option<String>,
    /// 字段值（可选）。
    pub value: Option<String>,
}

/// 媒体类型的消息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMedia {
    /// 文件类型（数字编码）。
    pub file_type: u8,
    /// 文件 URL。
    pub url: String,
    /// 是否由服务端发送消息。
    pub srv_send_msg: bool,
    /// 可选的文件数据（例如 base64 字符串）。
    pub file_data: Option<String>,
}
