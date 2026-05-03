pub(super) use crate::Result;
pub(super) use reqwest::Method;

pub(super) use super::client::OpenApiClient;
pub(super) use super::config::OpenApiPaths;
pub(super) use super::token::TokenProvider;
pub(super) use super::utils::{append_query, render_path, require_path};

mod channel;
mod guild;
mod interaction;
mod message;
mod open_api;
mod user;

pub use channel::{
    AudioApi, ChannelMessagesApi, ChannelPermissionsApi, ChannelsApi, ForumsApi, PinsApi,
    ReactionsApi, SchedulesApi,
};
pub use guild::{
    AnnouncesApi, ApiPermissionsApi, GuildsApi, MembersApi, MessageSettingsApi, MuteApi, RolesApi,
};
pub use interaction::InteractionsApi;
pub use message::{C2cMessagesApi, DmsMessagesApi, GroupMessagesApi, MediaApi};
pub use open_api::OpenApi;
pub use user::UsersApi;
