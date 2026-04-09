//! QQ bot SDK (webhook-first).

extern crate self as qqbot_sdk;

pub mod config;
mod error;
mod event;
mod events;
mod http;
mod middleware;
mod openapi;
mod router;
mod signature;

mod container;
mod handler;
mod macros;
mod replying;
mod runner;

pub use inventory;

pub use crate::error::{Error, Result};
pub use crate::event::{EventEnvelope, EventSchema};
pub use crate::events::common::{CommonMessage, FromCommonMessage};
pub use crate::events::payload::DispatchPayload;

pub use crate::http::{HttpClient, RetryPolicy};

pub use crate::openapi::{
    AnnouncesApi, ApiPermissionsApi, AuthConfig, C2cMessagesApi, ChannelPermissionsApi,
    ChannelsApi, ForumsApi, GuildsApi, HttpTokenProvider, InteractionsApi, MembersApi,
    MessageSettingsApi, MuteApi, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths, PinsApi,
    ReactionsApi, RolesApi, SchedulesApi, TokenManager, TokenProvider, UsersApi,
};
pub use crate::replying::{
    ReplyingArk, ReplyingArkKv, ReplyingEmbed, ReplyingEmbedField, ReplyingEmbedThumbnail,
    ReplyingMarkdown, ReplyingMarkdownParam, ReplyingMedia, ReplyingMessage, ReplyingType,
};

pub use crate::runner::run_application;
pub use crate::signature::{
    public_key_from_bot_secret, sign_webhook_validation, ReplayProtectionConfig,
    ReplayProtectionMode, SignatureConfig, SignatureEncoding, SignatureVerifier,
};

pub use crate::macros::command::{CommandDef, CommandHandleFn, CommandHandleFuture, CommandOutput};
pub use crate::config::{AppConfig, CredentialConfig, ListeningConfig};