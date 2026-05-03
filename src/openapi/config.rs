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
    /// 获取频道详情的路径模板。
    pub guild_get: Option<String>,
    /// 获取频道下子频道列表的路径模板。
    pub guild_channels: Option<String>,
    /// 创建频道下子频道的路径模板。
    pub guild_channel_create: Option<String>,
    /// 获取子频道详情的路径模板。
    pub channel_get: Option<String>,
    /// 修改子频道信息的路径模板。
    pub channel_update: Option<String>,
    /// 删除子频道的路径模板。
    pub channel_delete: Option<String>,
    /// 获取子频道在线人数的路径模板。
    pub channel_online_nums: Option<String>,
    /// 发送子频道消息的路径模板。
    pub channel_message_send: Option<String>,
    /// 获取子频道消息详情的路径模板。
    pub channel_message_get: Option<String>,
    /// 修改子频道消息的路径模板。
    pub channel_message_update: Option<String>,
    /// 撤回子频道消息的路径模板。
    pub channel_message_delete: Option<String>,
    /// 子频道音频控制的路径模板。
    pub channel_audio_control: Option<String>,
    /// 子频道麦克风控制的路径模板。
    pub channel_mic: Option<String>,
    /// 获取语音子频道成员的路径模板。
    pub channel_voice_members: Option<String>,
    /// 创建子频道公告的路径模板。
    pub channel_announces_create: Option<String>,
    /// 删除子频道公告的路径模板。
    pub channel_announces_delete: Option<String>,
    /// 创建私信会话的路径模板。
    pub dms_create: Option<String>,
    /// 发送私信消息的路径模板。
    pub dms_message_send: Option<String>,
    /// 撤回私信消息的路径模板。
    pub dms_message_delete: Option<String>,
    /// 撤回 C2C 单聊消息的路径模板。
    pub c2c_message_delete: Option<String>,
    /// 撤回群聊消息的路径模板。
    pub group_message_delete: Option<String>,
    /// 上传 C2C 富媒体资源的路径模板。
    pub c2c_file_upload: Option<String>,
    /// 上传群聊富媒体资源的路径模板。
    pub group_file_upload: Option<String>,
    /// 获取群聊成员列表的路径模板。
    pub group_members_list: Option<String>,
    /// 获取频道成员列表的路径模板。
    pub member_list: Option<String>,
    /// 获取频道成员详情的路径模板。
    pub member_get: Option<String>,
    /// 删除频道成员的路径模板。
    pub member_delete: Option<String>,
    /// 获取身份组成员列表的路径模板。
    pub role_members_list: Option<String>,
    /// 添加频道成员身份组的路径模板。
    pub role_member_add: Option<String>,
    /// 删除频道成员身份组的路径模板。
    pub role_member_delete: Option<String>,
    /// 互动事件回调确认的路径模板。
    pub interaction_put: Option<String>,
    /// 添加表情回应的路径模板。
    pub reaction_add: Option<String>,
    /// 删除表情回应的路径模板。
    pub reaction_delete: Option<String>,
    /// 获取表情回应用户列表的路径模板。
    pub reaction_users: Option<String>,
    /// 创建频道公告的路径模板。
    pub announces_create: Option<String>,
    /// 删除频道公告的路径模板。
    pub announces_delete: Option<String>,
    /// 获取 API 权限列表的路径模板。
    pub api_permissions_list: Option<String>,
    /// 创建 API 权限申请的路径模板。
    pub api_permissions_create: Option<String>,
    /// 获取用户子频道权限的路径模板。
    pub channel_permissions_get_user: Option<String>,
    /// 修改用户子频道权限的路径模板。
    pub channel_permissions_set_user: Option<String>,
    /// 获取身份组子频道权限的路径模板。
    pub channel_permissions_get_role: Option<String>,
    /// 修改身份组子频道权限的路径模板。
    pub channel_permissions_set_role: Option<String>,
    /// 获取精华消息列表的路径模板。
    pub pins_list: Option<String>,
    /// 添加精华消息的路径模板。
    pub pins_add: Option<String>,
    /// 删除精华消息的路径模板。
    pub pins_delete: Option<String>,
    /// 获取频道身份组列表的路径模板。
    pub role_list: Option<String>,
    /// 创建频道身份组的路径模板。
    pub role_create: Option<String>,
    /// 修改频道身份组的路径模板。
    pub role_update: Option<String>,
    /// 删除频道身份组的路径模板。
    pub role_delete: Option<String>,
    /// 获取子频道日程列表的路径模板。
    pub schedule_list: Option<String>,
    /// 获取子频道日程详情的路径模板。
    pub schedule_get: Option<String>,
    /// 创建子频道日程的路径模板。
    pub schedule_create: Option<String>,
    /// 修改子频道日程的路径模板。
    pub schedule_update: Option<String>,
    /// 删除子频道日程的路径模板。
    pub schedule_delete: Option<String>,
    /// 获取论坛帖子列表的路径模板。
    pub forum_threads_list: Option<String>,
    /// 获取论坛帖子详情的路径模板。
    pub forum_thread_get: Option<String>,
    /// 创建论坛帖子的路径模板。
    pub forum_thread_create: Option<String>,
    /// 删除论坛帖子的路径模板。
    pub forum_thread_delete: Option<String>,
    /// 全频道禁言的路径模板。
    pub mute_all: Option<String>,
    /// 指定频道成员禁言的路径模板。
    pub mute_user: Option<String>,
    /// 发送 C2C 单聊消息的路径模板。
    pub c2c_message_send: Option<String>,
    /// 发送群聊消息的路径模板。
    pub group_message_send: Option<String>,
    /// 获取频道消息设置的路径模板。
    pub message_setting_get: Option<String>,
    /// 获取当前用户信息的路径模板。
    pub user_me: Option<String>,
    /// 获取当前用户频道列表的路径模板。
    pub user_guilds: Option<String>,
}

impl OpenApiPaths {
    /// 创建 QQ 官方 OpenAPI 路径模板默认值。
    pub fn official_defaults() -> Self {
        Self {
            guild_get: Some("/guilds/{guild_id}".to_string()),
            guild_channels: Some("/guilds/{guild_id}/channels".to_string()),
            guild_channel_create: Some("/guilds/{guild_id}/channels".to_string()),
            channel_get: Some("/channels/{channel_id}".to_string()),
            channel_update: Some("/channels/{channel_id}".to_string()),
            channel_delete: Some("/channels/{channel_id}".to_string()),
            channel_online_nums: Some("/channels/{channel_id}/online_nums".to_string()),
            channel_message_send: Some("/channels/{channel_id}/messages".to_string()),
            channel_message_get: Some("/channels/{channel_id}/messages/{message_id}".to_string()),
            channel_message_update: Some(
                "/channels/{channel_id}/messages/{message_id}".to_string(),
            ),
            channel_message_delete: Some(
                "/channels/{channel_id}/messages/{message_id}".to_string(),
            ),
            channel_audio_control: Some("/channels/{channel_id}/audio".to_string()),
            channel_mic: Some("/channels/{channel_id}/mic".to_string()),
            channel_voice_members: Some("/channels/{channel_id}/voice/members".to_string()),
            channel_announces_create: Some("/channels/{channel_id}/announces".to_string()),
            channel_announces_delete: Some(
                "/channels/{channel_id}/announces/{message_id}".to_string(),
            ),
            dms_create: Some("/users/@me/dms".to_string()),
            dms_message_send: Some("/dms/{guild_id}/messages".to_string()),
            dms_message_delete: Some("/dms/{guild_id}/messages/{message_id}".to_string()),
            c2c_message_delete: Some("/v2/users/{openid}/messages/{message_id}".to_string()),
            group_message_delete: Some(
                "/v2/groups/{group_openid}/messages/{message_id}".to_string(),
            ),
            c2c_file_upload: Some("/v2/users/{openid}/files".to_string()),
            group_file_upload: Some("/v2/groups/{group_openid}/files".to_string()),
            group_members_list: Some("/v2/groups/{group_openid}/members".to_string()),
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
            group_message_send: Some("/v2/groups/{group_openid}/messages".to_string()),
            message_setting_get: Some("/guilds/{guild_id}/message/setting".to_string()),
            user_me: Some("/users/@me".to_string()),
            user_guilds: Some("/users/@me/guilds".to_string()),
        }
    }
}
