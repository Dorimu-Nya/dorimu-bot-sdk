use serde::{Deserialize, Serialize};
use crate::{C2cEventType, GroupEventType};
use crate::events::guild::event_type::GuildEventType;
use crate::events::interaction::event_type::InteractionEventType;
use crate::events::message_reaction::event_type::MessageReactionEventType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventType {
    C2cEventType(C2cEventType),
    GroupEventType(GroupEventType),
    GuildEventType(GuildEventType),
    InteractionEventType(InteractionEventType),
    MessageReactionEventType(MessageReactionEventType),
}
