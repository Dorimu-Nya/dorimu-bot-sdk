use super::events::c2c::event_type::C2cEventType;
use super::events::event_type::EventType;
use super::events::group::event_type::GroupEventType;
use super::events::guild::event_type::GuildEventType;
use super::events::interaction::event_type::InteractionEventType;
use super::events::message_reaction::event_type::MessageReactionEventType;
use super::events::payload::DispatchPayload;
use super::events::validation::{ValidationRequest, ValidationResponse};
use crate::container::COMMANDS;
use crate::events::common::CommonMessage;
use crate::macros::command::BoxDisplay;
use crate::ReplyingMessage;

pub fn handle_address_verify(
    req: ValidationRequest,
) -> Result<ValidationResponse, Box<dyn std::error::Error>> {
    let signature = crate::signature::sign_webhook_validation("", &req.event_ts, &req.plain_token)?;
    Ok(ValidationResponse {
        plain_token: req.plain_token,
        signature,
    })
}

pub async fn dispatch_event(payload: DispatchPayload) {
    match &payload.event {
        EventType::C2cEventType(event) => matching_c2c_event(event, &payload).await,
        EventType::GroupEventType(event) => matching_group_event(event, &payload).await,
        EventType::GuildEventType(event) => matching_guild_event(event, &payload).await,
        EventType::InteractionEventType(event) => matching_interaction_event(event).await,
        EventType::MessageReactionEventType(event) => matching_message_reaction_event(event).await,
    }
}

async fn matching_c2c_event(event: &C2cEventType, payload: &DispatchPayload) {
    match event {
        C2cEventType::C2cMessageCreate(message) => handle_messaging(message, payload).await,
        C2cEventType::FriendAdd(_) => {}
        C2cEventType::FriendDel(_) => {}
        C2cEventType::C2cMsgReject(_) => {}
        C2cEventType::C2cMsgReceive(_) => {}
    }
}

async fn matching_group_event(event: &GroupEventType, payload: &DispatchPayload) {
    match event {
        GroupEventType::GroupAtMessageCreate(message) => handle_messaging(message, payload).await,
        GroupEventType::GroupAddRobot(_) => {}
        GroupEventType::GroupDelRobot(_) => {}
        GroupEventType::GroupMsgReceive(_) => {}
        GroupEventType::GroupMsgReject(_) => {}
        GroupEventType::SubscribeMessageStatus => {}
    }
}

async fn matching_guild_event(event: &GuildEventType, _payload: &DispatchPayload) {
    match event {
        GuildEventType::AtMessageCreate(_message) => {}
        GuildEventType::PublicMessageDelete() => {}
        GuildEventType::DirectMessageCreate(_message) => {}
        GuildEventType::DirectMessageDelete() => {}
        GuildEventType::MessageReactionAdd => {}
        GuildEventType::MessageReactionRemove => {}
        GuildEventType::MessageAuditPass() => {}
        GuildEventType::MessageAuditReject() => {}
        GuildEventType::OpenForumThreadCreate(_) => {}
        GuildEventType::OpenForumPostCreate(_) => {}
        GuildEventType::OpenForumReplyCreate(_) => {}
        GuildEventType::OpenForumThreadUpdate(_) => {}
        GuildEventType::OpenForumPostDelete(_) => {}
        GuildEventType::OpenForumReplyDelete(_) => {}
        GuildEventType::OpenForumThreadDelete(_) => {}
        GuildEventType::GuildCreate(_) => {}
        GuildEventType::GuildUpdate(_) => {}
        GuildEventType::GuildDelete(_) => {}
        GuildEventType::ChannelCreate(_) => {}
        GuildEventType::ChannelUpdate(_) => {}
        GuildEventType::ChannelDelete(_) => {}
        GuildEventType::GuildMemberAdd(_) => {}
        GuildEventType::GuildMemberRemove(_) => {}
        GuildEventType::GuildMemberUpdate(_) => {}
        GuildEventType::AudioStart() => {}
        GuildEventType::AudioFinish() => {}
        GuildEventType::AudioOnMic() => {}
        GuildEventType::AudioOffMic() => {}
        GuildEventType::AudioOrLiveChannelMemberEnter(_) => {}
        GuildEventType::AudioOrLiveChannelMemberExit(_) => {}
    }
}

async fn matching_interaction_event(event: &InteractionEventType) {
    match event {
        InteractionEventType::InteractionCreate(_) => {}
    }
}

async fn matching_message_reaction_event(event: &MessageReactionEventType) {
    match event {
        MessageReactionEventType::MessageReactionAdd(_) => {}
        MessageReactionEventType::MessageReactionRemove(_) => {}
    }
}

async fn handle_messaging(message: &impl CommonMessage, _payload: &DispatchPayload) {
    match message.get_content() {
        None => {}
        Some(msg) => {
            let result: Vec<&str> = msg.split_whitespace().collect();
            if let Some(command_fn) = result.get(0).and_then(|cmd| COMMANDS.get(cmd)) {
                let res = command_fn(message).await;
                match res {
                    Ok(reply) => {
                        if let Some(reply) = reply {
                            //TODO openapi client发送消息
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }
}
