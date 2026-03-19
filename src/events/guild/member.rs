use serde::{Deserialize, Serialize};
use super::super::common::Member;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMemberEvent {
    #[serde(flatten)]
    pub member: Member,
    pub op_user_id: Option<String>,
}
