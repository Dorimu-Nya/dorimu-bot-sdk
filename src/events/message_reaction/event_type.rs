use serde::{Deserialize, Serialize};
use super::reaction::MessageReaction;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum MessageReactionEventType {
    MessageReactionAdd(MessageReaction),
    MessageReactionRemove(MessageReaction),
}