//! QQ bot SDK (webhook-first).

mod error;
mod event;
mod events;
mod http;
mod middleware;
mod openapi;
mod router;
mod signature;

mod runner;
mod handler;

pub use crate::error::{Error, Result};
pub use crate::event::{EventEnvelope, EventSchema};

pub use crate::http::{HttpClient, RetryPolicy};
// pub use crate::middleware::{Middleware, Next};
pub use crate::openapi::{
    AnnouncesApi, ApiPermissionsApi, AuthConfig, C2cMessagesApi, ChannelPermissionsApi,
    ChannelsApi, ForumsApi, GuildsApi, HttpTokenProvider, InteractionsApi, MembersApi,
    MessageSettingsApi, MuteApi, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths, PinsApi,
    ReactionsApi, RolesApi, SchedulesApi, TokenManager, TokenProvider, UsersApi,
};
// pub use crate::router::{EventHandler, EventRouter, HandlerFn};
pub use crate::signature::{
    public_key_from_bot_secret, sign_webhook_validation, ReplayProtectionConfig,
    ReplayProtectionMode, SignatureConfig, SignatureEncoding, SignatureVerifier,
};
pub use crate::runner::run_application;
// pub use crate::types::{EventContext, EventResponse};
// pub use crate::webhook::{
//     event_name_any, event_name_field, event_name_pointer, event_name_pointer_any,
//     webhook_validation_hook, EventNameExtractor, RawLogger, SignatureVerificationMode, WebhookApp,
//     WebhookConfig, WebhookHook,
// };
