//! QQ bot SDK (webhook-first).

mod error;
mod event;
mod events;
mod http;
mod middleware;
mod openapi;
mod router;
mod signature;
mod types;
mod webhook;

pub use crate::error::{Error, Result};
pub use crate::event::{EventEnvelope, EventSchema};
pub use crate::events::{
    AtGuildInfo, AtInfo, AtRoleInfo, AtUserInfo, Attachment, AudioOrLiveChannelMemberEvent,
    C2cAuthor, C2cMessage, C2cMessageScene, C2cMsgReceiveEvent, C2cMsgRejectEvent, ChannelEvent,
    ChannelInfo, Elem, Emoji, EmojiInfo, EventData, EventType, ForumAuditResult, ForumPostEvent,
    ForumPostInfo, ForumReplyEvent, ForumReplyInfo, ForumThreadEvent, ForumThreadInfo,
    FriendAddEvent, FriendDelEvent, GatewayPayload, GroupAddRobotEvent, GroupAtMessage,
    GroupAuthor, GroupDelRobotEvent, GroupMsgRejectEvent, GuildEvent, GuildMemberEvent, ImageElem,
    Interaction, InteractionData, InteractionResolved, Member, Message, MessageReaction,
    OpenForumEvent, Paragraph, ParagraphProps, PlatImage, PlatVideo, ReactionTarget, RichObject,
    RichText, RichTextValue, TextElem, TextInfo, TextProps, TypedEvent, UrlElem, UrlInfo, User,
    ValidationRequest, ValidationResponse, VideoElem,
};
pub use crate::http::{HttpClient, RetryPolicy};
pub use crate::middleware::{Middleware, Next};
pub use crate::openapi::{
    AnnouncesApi, ApiPermissionsApi, AuthConfig, C2cMessagesApi, ChannelPermissionsApi,
    ChannelsApi, ForumsApi, GuildsApi, HttpTokenProvider, InteractionsApi, MembersApi,
    MessageSettingsApi, MuteApi, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths, PinsApi,
    ReactionsApi, RolesApi, SchedulesApi, TokenManager, TokenProvider, UsersApi,
};
pub use crate::router::{EventHandler, EventRouter, HandlerFn};
pub use crate::signature::{
    public_key_from_bot_secret, sign_webhook_validation, ReplayProtectionConfig,
    ReplayProtectionMode, SignatureConfig, SignatureEncoding, SignatureVerifier,
};
pub use crate::types::{EventContext, EventResponse};
pub use crate::webhook::{
    event_name_any, event_name_field, event_name_pointer, event_name_pointer_any,
    webhook_validation_hook, EventNameExtractor, RawLogger, SignatureVerificationMode, WebhookApp,
    WebhookConfig, WebhookHook,
};
