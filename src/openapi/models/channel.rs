use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{Member, User};

/// GET /channels/{channel_id}/online_nums 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineNumsResponse {
    /// 在线成员数量。
    pub online_nums: u32,
}

/// 返回 [ChannelPermissions](model.md#channelpermissions) 对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPermissions {
    /// 子频道 ID。
    pub channel_id: String,
    /// 用户 ID。
    #[serde(default)]
    pub user_id: Option<String>,
    /// 身份组 ID。
    #[serde(default)]
    pub role_id: Option<String>,
    /// 权限位图字符串。
    pub permissions: String,
}

/// PUT /channels/{channel_id}/members/{user_id}/permissions 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyChannelPermissionsRequest {
    /// 字符串形式的位图表示赋予用户的权限。
    pub add: String,
    /// 字符串形式的位图表示删除用户的权限。
    pub remove: String,
}

/// 返回 [PinsMessage](model.md#PinsMessage) 对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinsMessage {
    /// 频道 ID。
    pub guild_id: String,
    /// 子频道 ID。
    pub channel_id: String,
    /// 精华消息 ID 列表。
    pub message_ids: Vec<String>,
}

/// GET /channels/{channel_id}/schedules 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchedulesQuery {
    /// 起始时间戳(**ms**)。
    #[serde(default)]
    pub since: Option<u64>,
}

/// 日程对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    /// 日程 ID。
    pub id: String,
    /// 日程名称。
    pub name: String,
    /// 开始时间戳（毫秒）。
    pub start_timestamp: String,
    /// 结束时间戳（毫秒）。
    pub end_timestamp: String,
    /// 创建人。
    #[serde(default)]
    pub creator: Option<Member>,
    /// 跳转子频道 ID。
    pub jump_channel_id: String,
    /// 提醒类型。
    pub remind_type: String,
}

/// 创建/修改日程时使用的日程对象，不需要带 `id`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleInput {
    /// 日程名称。
    pub name: String,
    /// 开始时间戳（毫秒）。
    pub start_timestamp: String,
    /// 结束时间戳（毫秒）。
    pub end_timestamp: String,
    /// 跳转子频道 ID。
    pub jump_channel_id: String,
    /// 提醒类型。
    pub remind_type: String,
}

/// POST/PATCH /channels/{channel_id}/schedules 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertScheduleRequest {
    /// 日程对象，不需要带 `id`。
    pub schedule: ScheduleInput,
}

/// GET /channels/{channel_id}/messages/{message_id}/reactions/{type}/{id} 查询参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactionUsersQuery {
    /// 分页标记。
    #[serde(default)]
    pub cookie: Option<String>,
    /// 分页大小。
    #[serde(default)]
    pub limit: Option<u32>,
}

/// 表情回应用户列表返回对象。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactionUsersResponse {
    /// 点赞用户列表（文档未给出结构，按常见返回定义）。
    #[serde(default)]
    pub users: Vec<User>,
    /// 是否拉取完毕（文档未给出结构，按常见返回定义）。
    #[serde(default)]
    pub is_end: Option<bool>,
    /// 下一页分页标记（文档未给出结构，按常见返回定义）。
    #[serde(default)]
    pub cookie: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// 帖子文本格式。
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum ThreadFormat {
    /// 普通文本。
    FormatText = 1,
    /// HTML。
    FormatHtml = 2,
    /// Markdown。
    FormatMarkdown = 3,
    /// JSON（content参数可参照[RichText](model.md#RichText)结构）。
    FormatJson = 4,
}

/// PUT /channels/{channel_id}/threads 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateThreadRequest {
    /// 帖子标题。
    pub title: String,
    /// 帖子内容。
    pub content: String,
    /// [帖子文本格式](#Format)。
    pub format: ThreadFormat,
}

/// PUT /channels/{channel_id}/threads 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateThreadResponse {
    /// 帖子任务ID。
    pub task_id: String,
    /// 发帖时间戳，单位：秒。
    pub create_time: String,
}

/// 帖子列表对象（返回值里面的content字段，可参照[RichText](model.md#RichText)结构）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Thread {
    /// 主帖ID（文档模型未展开，此字段按常见结构定义）。
    #[serde(default, alias = "id", alias = "thread_id")]
    pub thread_id: Option<String>,
    /// 帖子标题（文档模型未展开，此字段按常见结构定义）。
    #[serde(default)]
    pub title: Option<String>,
    /// 帖子内容（可参照 RichText 结构）。
    #[serde(default)]
    pub content: Option<Value>,
    /// 发表时间（文档模型未展开，此字段按常见结构定义）。
    #[serde(default)]
    pub date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// GET /channels/{channel_id}/threads 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadsListResponse {
    /// 帖子列表对象（返回值里面的content字段，可参照[RichText](model.md#RichText)结构）。
    pub threads: Vec<Thread>,
    /// 是否拉取完毕(0:否；1:是)。
    pub is_finish: u32,
}

/// GET /channels/{channel_id}/threads/{thread_id} 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadDetailResponse {
    /// 帖子详情对象（返回值里面的content字段，可参照[RichText](model.md#RichText)结构）。
    pub thread: Thread,
}
