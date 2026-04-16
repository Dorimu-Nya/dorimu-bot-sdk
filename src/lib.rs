//! QQ bot SDK (webhook-first).

extern crate self as qqbot_sdk;

#[cfg(feature = "events")]
mod events;
pub use crate::events::common::{CommonMessage, FromCommonMessage};
pub use crate::events::payload::DispatchPayload;

#[cfg(feature = "openapi")]
pub mod openapi;
pub use openapi::error::{Error, Result};
pub use openapi::http::{HttpClient, RetryPolicy};
pub use openapi::{
    AnnouncesApi, ApiPermissionsApi, AuthConfig, C2cMessagesApi, ChannelPermissionsApi,
    ChannelsApi, ForumsApi, GuildsApi, HttpTokenProvider, InteractionsApi, MembersApi,
    MessageSettingsApi, MuteApi, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths, PinsApi,
    ReactionsApi, RolesApi, SchedulesApi, TokenManager, TokenProvider, UsersApi,
};

#[cfg(feature = "signature")]
pub mod signature;
pub use signature::sign_webhook_validation;

#[cfg(feature = "app")]
pub mod app;
pub use app::commands::defining::{CommandDef, CommandHandleFn, CommandHandleFuture, CommandOutput};
pub use app::commands::replying::{ReplyingArk, ReplyingArkKv, ReplyingEmbed, ReplyingEmbedField, ReplyingEmbedThumbnail, ReplyingMarkdown, ReplyingMarkdownParam, ReplyingMedia, ReplyingMessage, ReplyingType};
pub use app::config::{AppConfig, CredentialConfig, ListeningConfig, SandboxConfig};

#[cfg(feature = "macros")]
pub use app::context::ContextStore;
#[cfg(feature = "macros")]
pub use inventory;

#[cfg(feature = "axum-runner")]
pub mod axum;
pub use axum::runner::{run_application, run_application_with_router};

pub use app::context::Context;