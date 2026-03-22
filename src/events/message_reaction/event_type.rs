use super::reaction::MessageReaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum MessageReactionEventType {
    MessageReactionAdd(MessageReaction),
    MessageReactionRemove(MessageReaction),
}
