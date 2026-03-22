use crate::RetryPolicy;
use http::header::HeaderName;

pub(crate) const OFFICIAL_API_BASE_URL: &str = "https://api.sgroup.qq.com";
pub(crate) const OFFICIAL_TOKEN_URL: &str = "https://bots.qq.com/app/getAppAccessToken";

/// OpenAPI 鉴权头配置。
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// 鉴权头名称，默认值为 `Authorization`。
    pub header_name: HeaderName,
    /// 鉴权值前缀，例如 `QQBot`。
    pub prefix: Option<String>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            header_name: http::header::AUTHORIZATION,
            prefix: None,
        }
    }
}

/// OpenAPI 客户端配置。
#[derive(Debug, Clone, Default)]
pub struct OpenApiConfig {
    /// OpenAPI 服务基础地址。
    pub base_url: String,
    /// 鉴权头配置。
    pub auth: AuthConfig,
    /// HTTP 重试策略。
    pub retry: RetryPolicy,
}

impl OpenApiConfig {
    /// 创建 QQ 官方 OpenAPI 默认配置。
    pub fn official() -> Self {
        Self {
            base_url: OFFICIAL_API_BASE_URL.to_string(),
            auth: AuthConfig {
                header_name: http::header::AUTHORIZATION,
                prefix: Some("QQBot".to_string()),
            },
            retry: RetryPolicy::default(),
        }
    }

    /// 优先从环境变量读取配置，否则回退到官方默认值。
    pub fn from_env_or_official() -> Self {
        let mut config = Self::official();
        if let Ok(base_url) = std::env::var("QQ_API_BASE_URL") {
            if !base_url.trim().is_empty() {
                config.base_url = base_url;
            }
        }
        config
    }
}

/// OpenAPI 路径模板集合。
#[derive(Debug, Clone, Default)]
pub struct OpenApiPaths {
    pub guild_get: Option<String>,
    pub guild_channels: Option<String>,
    pub channel_get: Option<String>,
    pub channel_online_nums: Option<String>,
    pub member_list: Option<String>,
    pub member_get: Option<String>,
    pub member_delete: Option<String>,
    pub role_members_list: Option<String>,
    pub role_member_add: Option<String>,
    pub role_member_delete: Option<String>,
    pub interaction_put: Option<String>,
    pub reaction_add: Option<String>,
    pub reaction_delete: Option<String>,
    pub reaction_users: Option<String>,
    pub announces_create: Option<String>,
    pub announces_delete: Option<String>,
    pub api_permissions_list: Option<String>,
    pub api_permissions_create: Option<String>,
    pub channel_permissions_get_user: Option<String>,
    pub channel_permissions_set_user: Option<String>,
    pub channel_permissions_get_role: Option<String>,
    pub channel_permissions_set_role: Option<String>,
    pub pins_list: Option<String>,
    pub pins_add: Option<String>,
    pub pins_delete: Option<String>,
    pub role_list: Option<String>,
    pub role_create: Option<String>,
    pub role_update: Option<String>,
    pub role_delete: Option<String>,
    pub schedule_list: Option<String>,
    pub schedule_get: Option<String>,
    pub schedule_create: Option<String>,
    pub schedule_update: Option<String>,
    pub schedule_delete: Option<String>,
    pub forum_threads_list: Option<String>,
    pub forum_thread_get: Option<String>,
    pub forum_thread_create: Option<String>,
    pub forum_thread_delete: Option<String>,
    pub mute_all: Option<String>,
    pub mute_user: Option<String>,
    pub c2c_message_send: Option<String>,
    pub message_setting_get: Option<String>,
    pub user_me: Option<String>,
    pub user_guilds: Option<String>,
}

impl OpenApiPaths {
    /// 创建 QQ 官方 OpenAPI 路径模板默认值。
    pub fn official_defaults() -> Self {
        Self {
            guild_get: Some("/guilds/{guild_id}".to_string()),
            guild_channels: Some("/guilds/{guild_id}/channels".to_string()),
            channel_get: Some("/channels/{channel_id}".to_string()),
            channel_online_nums: Some("/channels/{channel_id}/online_nums".to_string()),
            member_list: Some("/guilds/{guild_id}/members".to_string()),
            member_get: Some("/guilds/{guild_id}/members/{user_id}".to_string()),
            member_delete: Some("/guilds/{guild_id}/members/{user_id}".to_string()),
            role_members_list: Some("/guilds/{guild_id}/roles/{role_id}/members".to_string()),
            role_member_add: Some(
                "/guilds/{guild_id}/members/{user_id}/roles/{role_id}".to_string(),
            ),
            role_member_delete: Some(
                "/guilds/{guild_id}/members/{user_id}/roles/{role_id}".to_string(),
            ),
            interaction_put: Some("/interactions/{interaction_id}".to_string()),
            reaction_add: Some(
                "/channels/{channel_id}/messages/{message_id}/reactions/{type}/{id}".to_string(),
            ),
            reaction_delete: Some(
                "/channels/{channel_id}/messages/{message_id}/reactions/{type}/{id}".to_string(),
            ),
            reaction_users: Some(
                "/channels/{channel_id}/messages/{message_id}/reactions/{type}/{id}".to_string(),
            ),
            announces_create: Some("/guilds/{guild_id}/announces".to_string()),
            announces_delete: Some("/guilds/{guild_id}/announces/{message_id}".to_string()),
            api_permissions_list: Some("/guilds/{guild_id}/api_permission".to_string()),
            api_permissions_create: Some("/guilds/{guild_id}/api_permission/demand".to_string()),
            channel_permissions_get_user: Some(
                "/channels/{channel_id}/members/{user_id}/permissions".to_string(),
            ),
            channel_permissions_set_user: Some(
                "/channels/{channel_id}/members/{user_id}/permissions".to_string(),
            ),
            channel_permissions_get_role: Some(
                "/channels/{channel_id}/roles/{role_id}/permissions".to_string(),
            ),
            channel_permissions_set_role: Some(
                "/channels/{channel_id}/roles/{role_id}/permissions".to_string(),
            ),
            pins_list: Some("/channels/{channel_id}/pins".to_string()),
            pins_add: Some("/channels/{channel_id}/pins/{message_id}".to_string()),
            pins_delete: Some("/channels/{channel_id}/pins/{message_id}".to_string()),
            role_list: Some("/guilds/{guild_id}/roles".to_string()),
            role_create: Some("/guilds/{guild_id}/roles".to_string()),
            role_update: Some("/guilds/{guild_id}/roles/{role_id}".to_string()),
            role_delete: Some("/guilds/{guild_id}/roles/{role_id}".to_string()),
            schedule_list: Some("/channels/{channel_id}/schedules".to_string()),
            schedule_get: Some("/channels/{channel_id}/schedules/{schedule_id}".to_string()),
            schedule_create: Some("/channels/{channel_id}/schedules".to_string()),
            schedule_update: Some("/channels/{channel_id}/schedules/{schedule_id}".to_string()),
            schedule_delete: Some("/channels/{channel_id}/schedules/{schedule_id}".to_string()),
            forum_threads_list: Some("/channels/{channel_id}/threads".to_string()),
            forum_thread_get: Some("/channels/{channel_id}/threads/{thread_id}".to_string()),
            forum_thread_create: Some("/channels/{channel_id}/threads".to_string()),
            forum_thread_delete: Some("/channels/{channel_id}/threads/{thread_id}".to_string()),
            mute_all: Some("/guilds/{guild_id}/mute".to_string()),
            mute_user: Some("/guilds/{guild_id}/members/{user_id}/mute".to_string()),
            c2c_message_send: Some("/v2/users/{openid}/messages".to_string()),
            message_setting_get: Some("/guilds/{guild_id}/message/setting".to_string()),
            user_me: Some("/users/@me".to_string()),
            user_guilds: Some("/users/@me/guilds".to_string()),
        }
    }
}
