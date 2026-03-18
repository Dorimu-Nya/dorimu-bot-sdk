use serde::{Deserialize, Serialize};
use crate::{C2cEventType, GroupEventType, GuildEventType};


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventType {
    C2cEventType(C2cEventType),
    GroupEventType(GroupEventType),
    GuildEventType(GuildEventType),
}
