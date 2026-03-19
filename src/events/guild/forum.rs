use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumThreadEvent {
    pub guild_id: String,
    pub channel_id: String,
    pub author_id: String,
    pub thread_info: ForumThreadInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumPostEvent {
    pub guild_id: String,
    pub channel_id: String,
    pub author_id: String,
    pub post_info: ForumPostInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumReplyEvent {
    pub guild_id: String,
    pub channel_id: String,
    pub author_id: String,
    pub reply_info: ForumReplyInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumAuditResult {
    pub guild_id: String,
    pub channel_id: String,
    pub author_id: String,
    #[serde(rename = "type")]
    pub kind: Option<i64>,
    pub result: Option<i64>,
    pub err_msg: Option<String>,
    pub thread_id: Option<String>,
    pub post_id: Option<String>,
    pub reply_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenForumEvent {
    pub guild_id: String,
    pub channel_id: String,
    pub author_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForumThreadInfo {
    #[serde(default, alias = "id")]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub title: Option<RichTextValue>,
    #[serde(default)]
    pub content: Option<RichTextValue>,
    #[serde(default)]
    pub date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForumPostInfo {
    #[serde(default, alias = "id")]
    pub post_id: Option<String>,
    #[serde(default)]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub content: Option<RichTextValue>,
    #[serde(default)]
    pub date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForumReplyInfo {
    #[serde(default, alias = "id")]
    pub reply_id: Option<String>,
    #[serde(default)]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub post_id: Option<String>,
    #[serde(default)]
    pub content: Option<RichTextValue>,
    #[serde(default)]
    pub date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RichTextValue {
    Plain(String),
    Objects(Vec<RichObject>),
    RichText(RichText),
    Other(Value),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RichObject {
    #[serde(rename = "type")]
    pub kind: Option<i64>,
    #[serde(default)]
    pub text_info: Option<TextInfo>,
    #[serde(default)]
    pub at_info: Option<AtInfo>,
    #[serde(default)]
    pub url_info: Option<UrlInfo>,
    #[serde(default)]
    pub emoji_info: Option<EmojiInfo>,
    #[serde(default)]
    pub channel_info: Option<ChannelInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextInfo {
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AtInfo {
    #[serde(rename = "type")]
    pub kind: Option<i64>,
    pub user_info: Option<AtUserInfo>,
    pub role_info: Option<AtRoleInfo>,
    pub guild_info: Option<AtGuildInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AtUserInfo {
    pub id: Option<String>,
    pub nick: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AtRoleInfo {
    pub role_id: Option<String>,
    pub name: Option<String>,
    pub color: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AtGuildInfo {
    pub guild_id: Option<String>,
    pub guild_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UrlInfo {
    pub url: Option<String>,
    pub display_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmojiInfo {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub emoji_type: Option<i64>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChannelInfo {
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RichText {
    pub paragraphs: Option<Vec<Paragraph>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Paragraph {
    pub elems: Option<Vec<Elem>>,
    pub props: Option<ParagraphProps>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParagraphProps {
    pub alignment: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Elem {
    pub text: Option<TextElem>,
    pub image: Option<ImageElem>,
    pub video: Option<VideoElem>,
    pub url: Option<UrlElem>,
    #[serde(rename = "type")]
    pub elem_type: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextElem {
    pub text: Option<String>,
    pub props: Option<TextProps>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextProps {
    pub font_bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageElem {
    pub third_url: Option<String>,
    pub width_percent: Option<f64>,
    pub plat_image: Option<PlatImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatImage {
    pub url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VideoElem {
    pub third_url: Option<String>,
    pub plat_video: Option<PlatVideo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatVideo {
    pub url: Option<String>,
    pub duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UrlElem {
    pub url: Option<String>,
    pub desc: Option<String>,
}
