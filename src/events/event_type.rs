use super::c2c::event_type::C2cEventType;
use super::group::event_type::GroupEventType;
use super::guild::event_type::{ForumEventType, GuildEventType};
use super::interaction::event_type::InteractionEventType;
use super::message_reaction::event_type::MessageReactionEventType;
use serde::{Deserialize, Serialize};

/// 全部事件类型汇总分类
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventType {
    C2cEventType(C2cEventType),
    GroupEventType(GroupEventType),
    GuildEventType(GuildEventType),
    ForumEventType(ForumEventType),
    InteractionEventType(InteractionEventType),
    MessageReactionEventType(MessageReactionEventType),
}
