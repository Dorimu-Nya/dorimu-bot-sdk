use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

mod audio;
mod common;
mod forum;
mod group;
mod guild;
mod interaction;
mod member;
mod messages;
mod reaction;
mod user;
mod validation;

pub use audio::AudioOrLiveChannelMemberEvent;
pub use common::{Attachment, Member, User};
pub use forum::{
    AtGuildInfo, AtInfo, AtRoleInfo, AtUserInfo, ChannelInfo, Elem, EmojiInfo, ForumAuditResult,
    ForumPostEvent, ForumPostInfo, ForumReplyEvent, ForumReplyInfo, ForumThreadEvent,
    ForumThreadInfo, ImageElem, OpenForumEvent, Paragraph, ParagraphProps, PlatImage, PlatVideo,
    RichObject, RichText, RichTextValue, TextElem, TextInfo, TextProps, UrlElem, UrlInfo,
    VideoElem,
};
pub use group::{GroupAddRobotEvent, GroupDelRobotEvent, GroupMsgRejectEvent};
pub use guild::{ChannelEvent, GuildEvent};
pub use interaction::{Interaction, InteractionData, InteractionResolved};
pub use member::GuildMemberEvent;
pub use messages::{C2cAuthor, C2cMessage, C2cMessageScene, GroupAtMessage, GroupAuthor, Message};
pub use reaction::{Emoji, MessageReaction, ReactionTarget};
pub use user::{C2cMsgReceiveEvent, C2cMsgRejectEvent, FriendAddEvent, FriendDelEvent};
pub use validation::{ValidationRequest, ValidationResponse};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    C2cMessageCreate,
    GroupAtMessageCreate,
    DirectMessageCreate,
    AtMessageCreate,
    MessageCreate,
    FriendAdd,
    FriendDel,
    C2cMsgReject,
    C2cMsgReceive,
    GroupAddRobot,
    GroupDelRobot,
    GroupMsgReject,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    GuildMemberAdd,
    GuildMemberUpdate,
    GuildMemberRemove,
    AudioOrLiveChannelMemberEnter,
    AudioOrLiveChannelMemberExit,
    MessageReactionAdd,
    MessageReactionRemove,
    InteractionCreate,
    ForumThreadCreate,
    ForumThreadUpdate,
    ForumThreadDelete,
    ForumPostCreate,
    ForumPostDelete,
    ForumReplyCreate,
    ForumReplyDelete,
    ForumPublishAuditResult,
    OpenForumThreadCreate,
    OpenForumThreadUpdate,
    OpenForumThreadDelete,
    OpenForumPostCreate,
    OpenForumPostDelete,
    OpenForumReplyCreate,
    OpenForumReplyDelete,
    Other(String),
}

impl EventType {
    pub fn as_str(&self) -> &str {
        match self {
            EventType::C2cMessageCreate => "C2C_MESSAGE_CREATE",
            EventType::GroupAtMessageCreate => "GROUP_AT_MESSAGE_CREATE",
            EventType::DirectMessageCreate => "DIRECT_MESSAGE_CREATE",
            EventType::AtMessageCreate => "AT_MESSAGE_CREATE",
            EventType::MessageCreate => "MESSAGE_CREATE",
            EventType::FriendAdd => "FRIEND_ADD",
            EventType::FriendDel => "FRIEND_DEL",
            EventType::C2cMsgReject => "C2C_MSG_REJECT",
            EventType::C2cMsgReceive => "C2C_MSG_RECEIVE",
            EventType::GroupAddRobot => "GROUP_ADD_ROBOT",
            EventType::GroupDelRobot => "GROUP_DEL_ROBOT",
            EventType::GroupMsgReject => "GROUP_MSG_REJECT",
            EventType::GuildCreate => "GUILD_CREATE",
            EventType::GuildUpdate => "GUILD_UPDATE",
            EventType::GuildDelete => "GUILD_DELETE",
            EventType::ChannelCreate => "CHANNEL_CREATE",
            EventType::ChannelUpdate => "CHANNEL_UPDATE",
            EventType::ChannelDelete => "CHANNEL_DELETE",
            EventType::GuildMemberAdd => "GUILD_MEMBER_ADD",
            EventType::GuildMemberUpdate => "GUILD_MEMBER_UPDATE",
            EventType::GuildMemberRemove => "GUILD_MEMBER_REMOVE",
            EventType::AudioOrLiveChannelMemberEnter => "AUDIO_OR_LIVE_CHANNEL_MEMBER_ENTER",
            EventType::AudioOrLiveChannelMemberExit => "AUDIO_OR_LIVE_CHANNEL_MEMBER_EXIT",
            EventType::MessageReactionAdd => "MESSAGE_REACTION_ADD",
            EventType::MessageReactionRemove => "MESSAGE_REACTION_REMOVE",
            EventType::InteractionCreate => "INTERACTION_CREATE",
            EventType::ForumThreadCreate => "FORUM_THREAD_CREATE",
            EventType::ForumThreadUpdate => "FORUM_THREAD_UPDATE",
            EventType::ForumThreadDelete => "FORUM_THREAD_DELETE",
            EventType::ForumPostCreate => "FORUM_POST_CREATE",
            EventType::ForumPostDelete => "FORUM_POST_DELETE",
            EventType::ForumReplyCreate => "FORUM_REPLY_CREATE",
            EventType::ForumReplyDelete => "FORUM_REPLY_DELETE",
            EventType::ForumPublishAuditResult => "FORUM_PUBLISH_AUDIT_RESULT",
            EventType::OpenForumThreadCreate => "OPEN_FORUM_THREAD_CREATE",
            EventType::OpenForumThreadUpdate => "OPEN_FORUM_THREAD_UPDATE",
            EventType::OpenForumThreadDelete => "OPEN_FORUM_THREAD_DELETE",
            EventType::OpenForumPostCreate => "OPEN_FORUM_POST_CREATE",
            EventType::OpenForumPostDelete => "OPEN_FORUM_POST_DELETE",
            EventType::OpenForumReplyCreate => "OPEN_FORUM_REPLY_CREATE",
            EventType::OpenForumReplyDelete => "OPEN_FORUM_REPLY_DELETE",
            EventType::Other(value) => value.as_str(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Self {
        match value {
            "C2C_MESSAGE_CREATE" => EventType::C2cMessageCreate,
            "GROUP_AT_MESSAGE_CREATE" => EventType::GroupAtMessageCreate,
            "DIRECT_MESSAGE_CREATE" => EventType::DirectMessageCreate,
            "AT_MESSAGE_CREATE" => EventType::AtMessageCreate,
            "MESSAGE_CREATE" => EventType::MessageCreate,
            "FRIEND_ADD" => EventType::FriendAdd,
            "FRIEND_DEL" => EventType::FriendDel,
            "C2C_MSG_REJECT" => EventType::C2cMsgReject,
            "C2C_MSG_RECEIVE" => EventType::C2cMsgReceive,
            "GROUP_ADD_ROBOT" => EventType::GroupAddRobot,
            "GROUP_DEL_ROBOT" => EventType::GroupDelRobot,
            "GROUP_MSG_REJECT" => EventType::GroupMsgReject,
            "GUILD_CREATE" => EventType::GuildCreate,
            "GUILD_UPDATE" => EventType::GuildUpdate,
            "GUILD_DELETE" => EventType::GuildDelete,
            "CHANNEL_CREATE" => EventType::ChannelCreate,
            "CHANNEL_UPDATE" => EventType::ChannelUpdate,
            "CHANNEL_DELETE" => EventType::ChannelDelete,
            "GUILD_MEMBER_ADD" => EventType::GuildMemberAdd,
            "GUILD_MEMBER_UPDATE" => EventType::GuildMemberUpdate,
            "GUILD_MEMBER_REMOVE" => EventType::GuildMemberRemove,
            "AUDIO_OR_LIVE_CHANNEL_MEMBER_ENTER" => EventType::AudioOrLiveChannelMemberEnter,
            "AUDIO_OR_LIVE_CHANNEL_MEMBER_EXIT" => EventType::AudioOrLiveChannelMemberExit,
            "MESSAGE_REACTION_ADD" => EventType::MessageReactionAdd,
            "MESSAGE_REACTION_REMOVE" => EventType::MessageReactionRemove,
            "INTERACTION_CREATE" => EventType::InteractionCreate,
            "FORUM_THREAD_CREATE" => EventType::ForumThreadCreate,
            "FORUM_THREAD_UPDATE" => EventType::ForumThreadUpdate,
            "FORUM_THREAD_DELETE" => EventType::ForumThreadDelete,
            "FORUM_POST_CREATE" => EventType::ForumPostCreate,
            "FORUM_POST_DELETE" => EventType::ForumPostDelete,
            "FORUM_REPLY_CREATE" => EventType::ForumReplyCreate,
            "FORUM_REPLY_DELETE" => EventType::ForumReplyDelete,
            "FORUM_PUBLISH_AUDIT_RESULT" => EventType::ForumPublishAuditResult,
            "OPEN_FORUM_THREAD_CREATE" => EventType::OpenForumThreadCreate,
            "OPEN_FORUM_THREAD_UPDATE" => EventType::OpenForumThreadUpdate,
            "OPEN_FORUM_THREAD_DELETE" => EventType::OpenForumThreadDelete,
            "OPEN_FORUM_POST_CREATE" => EventType::OpenForumPostCreate,
            "OPEN_FORUM_POST_DELETE" => EventType::OpenForumPostDelete,
            "OPEN_FORUM_REPLY_CREATE" => EventType::OpenForumReplyCreate,
            "OPEN_FORUM_REPLY_DELETE" => EventType::OpenForumReplyDelete,
            other => EventType::Other(other.to_string()),
        }
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(EventType::from_str(&value))
    }
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayPayload<T> {
    #[serde(default)]
    pub id: Option<String>,
    pub op: u8,
    pub d: T,
    #[serde(default)]
    pub s: Option<u64>,
    #[serde(default)]
    pub t: Option<EventType>,
}

#[derive(Debug, Clone)]
pub struct TypedEvent {
    pub event_type: EventType,
    pub data: EventData,
    pub id: Option<String>,
    pub op: u8,
    pub seq: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum EventData {
    C2cMessage(C2cMessage),
    GroupAtMessage(GroupAtMessage),
    DirectMessage(Message),
    AtMessage(Message),
    Message(Message),
    FriendAdd(FriendAddEvent),
    FriendDel(FriendDelEvent),
    C2cMsgReject(C2cMsgRejectEvent),
    C2cMsgReceive(C2cMsgReceiveEvent),
    GroupAddRobot(GroupAddRobotEvent),
    GroupDelRobot(GroupDelRobotEvent),
    GroupMsgReject(GroupMsgRejectEvent),
    GuildEvent(GuildEvent),
    ChannelEvent(ChannelEvent),
    GuildMemberEvent(GuildMemberEvent),
    AudioOrLiveChannelMemberEnter(AudioOrLiveChannelMemberEvent),
    AudioOrLiveChannelMemberExit(AudioOrLiveChannelMemberEvent),
    MessageReactionAdd(MessageReaction),
    MessageReactionRemove(MessageReaction),
    InteractionCreate(Interaction),
    ForumThreadEvent(ForumThreadEvent),
    ForumPostEvent(ForumPostEvent),
    ForumReplyEvent(ForumReplyEvent),
    ForumAuditResult(ForumAuditResult),
    OpenForumEvent(OpenForumEvent),
    Unknown(Value),
}

impl TypedEvent {
    pub fn from_value(payload: &Value) -> Result<Self> {
        let gateway: GatewayPayload<Value> =
            serde_json::from_value(payload.clone()).map_err(Error::Json)?;
        let event_type = gateway.t.clone().ok_or(Error::EventNameNotFound)?;
        let data = match &event_type {
            EventType::C2cMessageCreate => EventData::C2cMessage(parse_data(&gateway.d)?),
            EventType::GroupAtMessageCreate => EventData::GroupAtMessage(parse_data(&gateway.d)?),
            EventType::DirectMessageCreate => EventData::DirectMessage(parse_data(&gateway.d)?),
            EventType::AtMessageCreate => EventData::AtMessage(parse_data(&gateway.d)?),
            EventType::MessageCreate => EventData::Message(parse_data(&gateway.d)?),
            EventType::FriendAdd => EventData::FriendAdd(parse_data(&gateway.d)?),
            EventType::FriendDel => EventData::FriendDel(parse_data(&gateway.d)?),
            EventType::C2cMsgReject => EventData::C2cMsgReject(parse_data(&gateway.d)?),
            EventType::C2cMsgReceive => EventData::C2cMsgReceive(parse_data(&gateway.d)?),
            EventType::GroupAddRobot => EventData::GroupAddRobot(parse_data(&gateway.d)?),
            EventType::GroupDelRobot => EventData::GroupDelRobot(parse_data(&gateway.d)?),
            EventType::GroupMsgReject => EventData::GroupMsgReject(parse_data(&gateway.d)?),
            EventType::GuildCreate | EventType::GuildUpdate | EventType::GuildDelete => {
                EventData::GuildEvent(parse_data(&gateway.d)?)
            }
            EventType::ChannelCreate | EventType::ChannelUpdate | EventType::ChannelDelete => {
                EventData::ChannelEvent(parse_data(&gateway.d)?)
            }
            EventType::GuildMemberAdd
            | EventType::GuildMemberUpdate
            | EventType::GuildMemberRemove => EventData::GuildMemberEvent(parse_data(&gateway.d)?),
            EventType::AudioOrLiveChannelMemberEnter => {
                EventData::AudioOrLiveChannelMemberEnter(parse_data(&gateway.d)?)
            }
            EventType::AudioOrLiveChannelMemberExit => {
                EventData::AudioOrLiveChannelMemberExit(parse_data(&gateway.d)?)
            }
            EventType::MessageReactionAdd => EventData::MessageReactionAdd(parse_data(&gateway.d)?),
            EventType::MessageReactionRemove => {
                EventData::MessageReactionRemove(parse_data(&gateway.d)?)
            }
            EventType::InteractionCreate => EventData::InteractionCreate(parse_data(&gateway.d)?),
            EventType::ForumThreadCreate
            | EventType::ForumThreadUpdate
            | EventType::ForumThreadDelete => EventData::ForumThreadEvent(parse_data(&gateway.d)?),
            EventType::ForumPostCreate | EventType::ForumPostDelete => {
                EventData::ForumPostEvent(parse_data(&gateway.d)?)
            }
            EventType::ForumReplyCreate | EventType::ForumReplyDelete => {
                EventData::ForumReplyEvent(parse_data(&gateway.d)?)
            }
            EventType::ForumPublishAuditResult => {
                EventData::ForumAuditResult(parse_data(&gateway.d)?)
            }
            EventType::OpenForumThreadCreate
            | EventType::OpenForumThreadUpdate
            | EventType::OpenForumThreadDelete
            | EventType::OpenForumPostCreate
            | EventType::OpenForumPostDelete
            | EventType::OpenForumReplyCreate
            | EventType::OpenForumReplyDelete => EventData::OpenForumEvent(parse_data(&gateway.d)?),
            EventType::Other(_) => EventData::Unknown(gateway.d.clone()),
        };

        Ok(Self {
            event_type,
            data,
            id: gateway.id,
            op: gateway.op,
            seq: gateway.s,
        })
    }
}

fn parse_data<T: serde::de::DeserializeOwned>(value: &Value) -> Result<T> {
    serde_json::from_value(value.clone()).map_err(Error::Json)
}
