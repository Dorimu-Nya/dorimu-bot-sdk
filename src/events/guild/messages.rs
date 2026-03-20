use serde::{Deserialize, Serialize};
use super::member::{Member, User};
use super::super::common::{Attachment};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMessages {
    pub id: String,
    pub channel_id: Option<String>,
    pub guild_id: Option<String>,
    pub content: Option<String>,
    pub timestamp: Option<String>,
    pub author: Option<User>,
    pub member: Option<Member>,
    pub attachments: Option<Vec<Attachment>>,
    pub seq: Option<u64>,
    pub seq_in_channel: Option<u64>,
}
