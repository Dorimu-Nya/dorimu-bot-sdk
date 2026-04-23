use super::commands::replying::ReplyingMessage;
use super::App;
use crate::events::c2c::event_type::C2cEventType;
use crate::events::common::CommonMessage;
use crate::events::event_type::EventType;
use crate::events::group::event_type::GroupEventType;
use crate::events::guild::event_type::GuildEventType;
use crate::events::interaction::event_type::InteractionEventType;
use crate::events::message_reaction::event_type::MessageReactionEventType;
use crate::events::payload::DispatchPayload;
use crate::events::payload::WebhookPayload;
use crate::events::validation::{ValidationRequest, ValidationResponse};
use serde_json::json;
use tracing::{debug, error, warn};

impl App {
    // webhook的第一层的对t字段的处理
    pub async fn webhook_handler(&self, payload: WebhookPayload) -> Option<ValidationResponse> {
        debug!("收到Webhook事件: {:?}", payload);
        match payload {
            WebhookPayload::Dispatch(payload) => {
                self.dispatch_event(payload).await;
                None
            }
            WebhookPayload::HttpCallbackAck(_) => None,
            WebhookPayload::WebhookAddressVerify(payload) => {
                let res = self.handle_address_verify(payload.d).unwrap();
                Some(res)
            }
        }
    }

    /// 处理腾讯端请求签名校验
    pub fn handle_address_verify(
        &self,
        req: ValidationRequest,
    ) -> Result<ValidationResponse, Box<dyn std::error::Error>> {
        let signature = crate::signature::sign_webhook_validation(
            &self.credential.secret,
            &req.event_ts,
            &req.plain_token,
        )?;
        Ok(ValidationResponse {
            plain_token: req.plain_token,
            signature,
        })
    }

    /// 当webhook opcode为0时，处理事件分发
    async fn dispatch_event(&self, payload: DispatchPayload) {
        match &payload.event {
            EventType::C2cEventType(event) => self.matching_c2c_event(event, &payload).await,
            EventType::GroupEventType(event) => self.matching_group_event(event, &payload).await,
            EventType::GuildEventType(event) => self.matching_guild_event(event, &payload).await,
            EventType::InteractionEventType(event) => self.matching_interaction_event(event).await,
            EventType::MessageReactionEventType(event) => {
                self.matching_message_reaction_event(event).await
            },
            &EventType::ForumEventType(_) => {}
        }
    }

    /// 处理 C2C 事件...
    async fn matching_c2c_event(&self, event: &C2cEventType, payload: &DispatchPayload) {
        match event {
            // sheip9(2026/4/9): 设想的处理逻辑是 先调用c2c专属的command（下方群组消息相关同理），若无再查找通用的command，若无对应的command，则广播到任何监听消息事件的方法，现在只实现了通用command, 后续再慢慢迭代
            C2cEventType::C2cMessageCreate(message) => {
                let reply = self.handle_messaging(message, payload).await;
                if let Some(reply) = reply {
                    // todo: 手写对象不靠谱，还是要重构下openapi
                    let body = json!({
                        "msg_id": message.id,
                        "msg_seq": message.msg_seq.unwrap_or(1),
                        "msg_type": reply.to_msg_type(),
                        "content": reply,
                    });

                    let _ = self
                        .get_prod_client()
                        .c2c_messages()
                        .send(&message.author.user_openid, &body)
                        .await;
                }
            }
            C2cEventType::FriendAdd(_) => {}
            C2cEventType::FriendDel(_) => {}
            C2cEventType::C2cMsgReject(_) => {}
            C2cEventType::C2cMsgReceive(_) => {}
        }
    }

    /// 处理群事件...
    async fn matching_group_event(&self, event: &GroupEventType, payload: &DispatchPayload) {
        match event {
            GroupEventType::GroupAtMessageCreate(message) => {
                let reply = self.handle_messaging(message, payload).await;
                if let Some(reply) = reply {
                    let _body = json!({
                        "msg_id": message.id,
                        "msg_seq": message.msg_seq.unwrap_or(1),
                        "msg_type": reply.to_msg_type(),
                        "content": reply,
                    });

                    // TODO: openapi部分缺了群组的api
                    // let _ = self.get_api_client()
                    //     .
                    //     .send(&message.author.member_openid, &body)
                    //     .await;
                }
            }
            GroupEventType::GroupAddRobot(_) => {}
            GroupEventType::GroupDelRobot(_) => {}
            GroupEventType::GroupMsgReceive(_) => {}
            GroupEventType::GroupMsgReject(_) => {}
            GroupEventType::SubscribeMessageStatus => {}
        }
    }

    /// 处理频道事件...
    async fn matching_guild_event(&self, event: &GuildEventType, _payload: &DispatchPayload) {
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

    /// 处理交互事件...
    async fn matching_interaction_event(&self, event: &InteractionEventType) {
        match event {
            InteractionEventType::InteractionCreate(_) => {}
        }
    }

    /// 处理消息反应事件...
    async fn matching_message_reaction_event(&self, event: &MessageReactionEventType) {
        match event {
            MessageReactionEventType::MessageReactionAdd(_) => {}
            MessageReactionEventType::MessageReactionRemove(_) => {}
        }
    }

    /// 处理消息指令等
    ///
    /// 这个方法会：
    /// 1. 解析消息内容，提取命令
    /// 2. 从命令表中查找对应的处理函数
    /// 3. 创建依赖容器并传递给命令处理函数
    /// 4. 执行命令并返回回复消息
    async fn handle_messaging(
        &self,
        message: &impl CommonMessage,
        _payload: &DispatchPayload,
    ) -> Option<ReplyingMessage> {
        match message.get_content() {
            None => None,
            Some(msg) => {
                let result: Vec<&str> = msg.split_whitespace().collect();
                if let Some(command_fn) = result.get(0).and_then(|cmd| self.commands.get(cmd)) {
                    let container = &self.dependency_container;

                    // 调用命令处理函数，传递消息和容器
                    let res = command_fn(message, container).await;
                    match res {
                        Ok(reply) => reply,
                        Err(err) => {
                            error!("处理指令{}出错: {}", msg, err);
                            None
                        }
                    }
                } else {
                    warn!("未知指令: {}", msg);
                    None
                }
            }
        }
    }
}
