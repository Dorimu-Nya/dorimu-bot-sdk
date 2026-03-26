use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingMarkdown {
    pub content: Option<String>,
    pub custom_template_id: Option<String>,
    pub params: Option<Vec<ReplyingMarkdownParam>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingMarkdownParam {
    pub key: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingArk {
    pub template_id: u64,
    pub kv: Vec<ReplyingArkKv>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingArkKv {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingEmbed {
    pub title: Option<String>,
    pub prompt: Option<String>,
    pub thumbnail: Option<ReplyingEmbedThumbnail>,
    pub fields: Option<Vec<ReplyingEmbedField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingEmbedThumbnail {
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingEmbedField {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyingMedia {
    pub file_type: u8,
    pub url: String,
    pub srv_send_msg: bool,
    pub file_data: Option<String>,
}

pub enum ReplyingType {
    C2c,
    Group,
}

pub enum ReplyingMessage {
    Text(String),
    Markdown(ReplyingMarkdown),
    Ark(ReplyingArk),
    Embed(ReplyingEmbed),
    Media(ReplyingMedia),
}

impl ReplyingMessage {
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
