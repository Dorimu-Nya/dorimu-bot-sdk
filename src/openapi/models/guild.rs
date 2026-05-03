use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::{Member, Role};

/// DELETE /guilds/{guild_id}/members/{user_id} 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteMemberOptions {
    /// 删除成员的同时，将该用户添加到频道黑名单中。
    #[serde(default)]
    pub add_blacklist: Option<bool>,
    /// 删除成员的同时，撤回该成员的消息，可以指定撤回消息的时间范围。
    #[serde(default)]
    pub delete_history_msg_days: Option<i32>,
}

/// GET /guilds/{guild_id}/roles/{role_id}/members 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoleMembersQuery {
    /// 将上一次回包中`next`填入， 如果是第一次请求填 0，默认为 0。
    #[serde(default)]
    pub start_index: Option<String>,
    /// 分页大小，1-400，默认是 1。成员较多的频道尽量使用较大的`limit`值，以减少请求数。
    #[serde(default)]
    pub limit: Option<u32>,
}

/// GET /guilds/{guild_id}/roles/{role_id}/members 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleMembersResponse {
    /// 一组用户信息对象。
    pub data: Vec<Member>,
    /// 下一次请求的分页标识。
    pub next: String,
}

/// GET /guilds/{guild_id}/roles 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRolesResponse {
    /// 频道 ID。
    pub guild_id: String,
    /// 一组频道身份组对象。
    pub roles: Vec<Role>,
    /// 默认分组上限。
    pub role_num_limit: String,
}

/// POST /guilds/{guild_id}/roles 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateRoleRequest {
    /// 名称(非必填)。
    #[serde(default)]
    pub name: Option<String>,
    /// ARGB 的 HEX 十六进制颜色值转换后的十进制数值(非必填)。
    #[serde(default)]
    pub color: Option<u32>,
    /// 在成员列表中单独展示: 0-否, 1-是(非必填)。
    #[serde(default)]
    pub hoist: Option<i32>,
}

/// POST /guilds/{guild_id}/roles 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleResponse {
    /// 身份组 ID。
    pub role_id: String,
    /// 所创建的频道身份组对象。
    pub role: Role,
}

/// PATCH /guilds/{guild_id}/roles/{role_id} 请求参数。
pub type UpdateRoleRequest = CreateRoleRequest;

/// PATCH /guilds/{guild_id}/roles/{role_id} 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleResponse {
    /// 频道 ID。
    pub guild_id: String,
    /// 身份组 ID。
    pub role_id: String,
    /// 修改后的频道身份组对象。
    pub role: Role,
}

/// 接收一个只填充了子频道 id 字段的对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleMemberChannel {
    /// 子频道 ID。
    pub id: String,
}

/// PUT/DELETE 频道身份组成员操作请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleMemberActionRequest {
    /// 接收一个只填充了子频道 id 字段的对象。
    pub channel: RoleMemberChannel,
}

/// 推荐子频道对象。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecommendChannel {
    /// 子频道 ID（文档模型未展开，此字段按常见结构定义）。
    #[serde(default)]
    pub channel_id: Option<String>,
    /// 推荐语（文档模型未展开，此字段按常见结构定义）。
    #[serde(default)]
    pub introduce: Option<String>,
    /// OpenAPI 返回的未显式建模字段。
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// POST /guilds/{guild_id}/announces 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAnnouncesRequest {
    /// 选填，消息 id，message_id 有值则优选将某条消息设置为成员公告。
    #[serde(default)]
    pub message_id: Option<String>,
    /// 选填，子频道 id，message_id 有值则为必填。
    #[serde(default)]
    pub channel_id: Option<String>,
    /// 选填，公告类别 0:成员公告，1:欢迎公告，默认为成员公告。
    #[serde(default)]
    pub announces_type: Option<u32>,
    /// 选填，推荐子频道列表，会一次全部替换推荐子频道列表。
    #[serde(default)]
    pub recommend_channels: Vec<RecommendChannel>,
}

/// 返回 [Announces](model.md#Announces) 对象。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Announces {
    /// 频道 ID。
    #[serde(default)]
    pub guild_id: Option<String>,
    /// 子频道 ID。
    #[serde(default)]
    pub channel_id: Option<String>,
    /// 公告消息 ID。
    #[serde(default)]
    pub message_id: Option<String>,
    /// 公告类别 0:成员公告，1:欢迎公告。
    #[serde(default)]
    pub announces_type: Option<u32>,
    /// 推荐子频道列表。
    #[serde(default)]
    pub recommend_channels: Vec<RecommendChannel>,
}

/// APIPermission 对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPermission {
    /// API 路径。
    pub path: String,
    /// HTTP 方法。
    pub method: String,
    /// API 描述。
    pub desc: String,
    /// 授权状态。
    pub auth_status: i32,
}

/// GET /guilds/{guild_id}/api_permission 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPermissionsResponse {
    /// 机器人可用权限列表。
    pub apis: Vec<ApiPermission>,
}

/// api 权限需求标识对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPermissionDemandIdentify {
    /// API 路径。
    pub path: String,
    /// HTTP 方法。
    pub method: String,
}

/// POST /guilds/{guild_id}/api_permission/demand 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiPermissionDemandRequest {
    /// 授权链接发送的子频道 id。
    pub channel_id: String,
    /// api 权限需求标识对象。
    pub api_identify: ApiPermissionDemandIdentify,
    /// 机器人申请对应的 API 接口权限后可以使用功能的描述。
    pub desc: String,
}

/// 返回 [APIPermissionDemand](model.md#APIPermissionDemand) 对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPermissionDemand {
    /// 频道 ID。
    pub guild_id: String,
    /// 子频道 ID。
    pub channel_id: String,
    /// API 权限需求标识对象。
    pub api_identify: ApiPermissionDemandIdentify,
    /// 授权申请标题。
    pub title: String,
    /// 授权申请描述。
    pub desc: String,
}

/// PATCH /guilds/{guild_id}/mute 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GuildMuteRequest {
    /// 禁言到期时间戳，绝对时间戳，单位：秒（与 mute_seconds 字段同时赋值的话，以该字段为准）。
    #[serde(default)]
    pub mute_end_timestamp: Option<String>,
    /// 禁言多少秒（两个字段二选一，默认以 mute_end_timestamp 为准）。
    #[serde(default)]
    pub mute_seconds: Option<String>,
}

/// PATCH /guilds/{guild_id}/mute（批量）请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GuildMuteMultiMemberRequest {
    /// 禁言到期时间戳，绝对时间戳，单位：秒（与 mute_seconds 字段同时赋值的话，以该字段为准）。
    #[serde(default)]
    pub mute_end_timestamp: Option<String>,
    /// 禁言多少秒（两个字段二选一，默认以 mute_end_timestamp 为准）。
    #[serde(default)]
    pub mute_seconds: Option<String>,
    /// 禁言成员的user_id列表，即[User](../../../openapi/user/model.md#user)的id。
    #[serde(default)]
    pub user_ids: Vec<String>,
}

/// PATCH /guilds/{guild_id}/mute（批量）返回参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GuildMuteMultiMemberResponse {
    /// 设置成功的成员 user_ids。
    #[serde(default)]
    pub user_ids: Vec<String>,
}

/// 返回 [MessageSetting](model.md#MessageSetting) 对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSetting {
    /// 是否禁止主动创建私信。
    pub disable_create_dm: bool,
    /// 是否禁止主动推送消息。
    pub disable_push_msg: bool,
    /// 允许推送的子频道 ID 列表。
    pub channel_ids: Vec<String>,
    /// 单次最多推送子频道数量。
    pub channel_push_max_num: u32,
}

/// GET /guilds/{guild_id}/members 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MembersQuery {
    /// 上一次回包中最后一个`member`的`user id`， 如果是第一次请求填 0，默认为 0。
    #[serde(default)]
    pub after: Option<String>,
    /// 分页大小，1-400，默认是 1。成员较多的频道尽量使用较大的`limit`值，以减少请求数。
    #[serde(default)]
    pub limit: Option<u32>,
}
