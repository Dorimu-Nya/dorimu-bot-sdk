use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub content_type: Option<String>,
    pub filename: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
    pub size: Option<i64>,
    pub url: Option<String>,
}

pub trait Message {
    fn get_content(&self) -> Option<String>;

    fn get_author_openid(&self) -> String;
}

