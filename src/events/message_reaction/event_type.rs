use serde::{Deserialize, Serialize};
use crate::MessageReaction;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum MessageReactionEventType {
    MessageReactionAdd(MessageReaction),
    MessageReactionRemove(MessageReaction),
}