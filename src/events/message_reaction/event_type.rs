use super::reaction::MessageReaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum MessageReactionEventType {
    /// 为消息添加表情表态
    #[serde(rename = "MESSAGE_REACTION_ADD")]
    MessageReactionAdd(MessageReaction),
    /// 为消息删除表情表态
    #[serde(rename = "MESSAGE_REACTION_REMOVE")]
    MessageReactionRemove(MessageReaction),
}
