use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{Member, User};

/// 消息类型：0 是文本，2 是 markdown，3 ark，4 embed，7 media 富媒体。
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    /// 文本。
    Text = 0,
    /// Markdown。
    Markdown = 2,
    /// Ark。
    Ark = 3,
    /// Embed。
    Embed = 4,
    /// 富媒体。
    Media = 7,
}

/// 任意 JSON 对象。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct JsonObject(pub BTreeMap<String, Value>);

/// 消息引用对象。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageReference {
    /// 被引用的消息 ID。
    #[serde(default)]
    pub message_id: Option<String>,
    /// OpenAPI 返回的未显式建模字段。
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// 消息附件对象。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageAttachment {
    /// 附件 ID。
    #[serde(default)]
    pub id: Option<String>,
    /// 文件名。
    #[serde(default)]
    pub filename: Option<String>,
    /// 文件大小。
    #[serde(default)]
    pub size: Option<u64>,
    /// 附件 URL。
    #[serde(default)]
    pub url: Option<String>,
    /// OpenAPI 返回的未显式建模字段。
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// 频道消息对象。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    /// 消息 ID。
    #[serde(default)]
    pub id: Option<String>,
    /// 子频道 ID。
    #[serde(default)]
    pub channel_id: Option<String>,
    /// 频道 ID。
    #[serde(default)]
    pub guild_id: Option<String>,
    /// 消息内容。
    #[serde(default)]
    pub content: Option<String>,
    /// 消息发送时间。
    #[serde(default)]
    pub timestamp: Option<String>,
    /// 消息编辑时间。
    #[serde(default)]
    pub edited_timestamp: Option<String>,
    /// 消息创建者。
    #[serde(default)]
    pub author: Option<User>,
    /// 成员信息。
    #[serde(default)]
    pub member: Option<Member>,
    /// 消息引用。
    #[serde(default)]
    pub message_reference: Option<MessageReference>,
    /// 被提及用户列表。
    #[serde(default)]
    pub mentions: Vec<User>,
    /// 附件列表。
    #[serde(default)]
    pub attachments: Vec<MessageAttachment>,
    /// 消息序号。
    #[serde(default)]
    pub seq: Option<u64>,
    /// 子频道内消息序号。
    #[serde(default)]
    pub seq_in_channel: Option<String>,
    /// OpenAPI 返回的未显式建模字段。
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// POST /v2/users/{openid}/messages 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequest {
    /// 文本内容。
    #[serde(default)]
    pub content: Option<String>,
    /// 消息类型：0 是文本，2 是 markdown， 3 ark，4 embed，7 media 富媒体。
    pub msg_type: MessageType,
    /// [Markdown](../type/markdown.md#数据结构与协议)对象。
    #[serde(default)]
    pub markdown: Option<MessageMarkdown>,
    /// [Keyboard](../interaction/msg-btn.md#数据结构与协议)对象。
    #[serde(default)]
    pub keyboard: Option<Keyboard>,
    /// [Ark](../type/ark.md#数据结构与协议)对象。
    #[serde(default)]
    pub ark: Option<MessageArk>,
    /// [富媒体单聊](./rich-media.md#用于单聊)的file_info。
    #[serde(default)]
    pub media: Option<MessageMedia>,
    /// Embed消息
    #[serde(default)]
    pub embed: Option<MessageEmbed>,
    /// 【暂未支持】消息引用。
    #[serde(default)]
    pub message_reference: Option<JsonObject>,
    /// 前置收到的事件 ID，用于发送被动消息，支持事件："INTERACTION_CREATE"、"C2C_MSG_RECEIVE"、"FRIEND_ADD"。
    #[serde(default)]
    pub event_id: Option<String>,
    /// 前置收到的用户发送过来的消息 ID，用于发送被动（回复）消息。
    #[serde(default)]
    pub msg_id: Option<String>,
    /// 回复消息的序号，与 msg_id 联合使用，避免相同消息id回复重复发送，不填默认是1。相同的 msg_id + msg_seq 重复发送会失败。
    #[serde(default)]
    pub msg_seq: Option<u64>,
}

impl Default for SendMessageRequest {
    fn default() -> Self {
        Self {
            msg_type: MessageType::Text,
            content: None,
            markdown: None,
            keyboard: None,
            ark: None,
            media: None,
            embed: None,
            message_reference: None,
            event_id: None,
            msg_id: None,
            msg_seq: None,
        }
    }
}

/// POST /v2/users/{openid}/messages 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    /// 消息唯一ID。
    pub id: Option<String>,
    /// 发送时间。
    pub timestamp: Option<String>,
    /// 错误码，接口返回错误时可能存在，例如：40034028。
    #[serde(default)]
    pub code: Option<i64>,
    /// 错误码，接口返回错误时可能存在，例如：40034028。
    #[serde(default)]
    pub err_code: Option<i64>,
    /// 错误信息，接口返回错误时可能存在。
    #[serde(default)]
    pub message: Option<String>,
    /// 链路追踪 ID，接口返回错误时可能存在。
    #[serde(default)]
    pub trace_id: Option<String>,
    /// 额外字段，避免接口新增或返回未声明字段时丢失，便于通过 Debug/日志输出排查。
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, Value>,
}

/// Markdown 消息
///
/// https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/type/markdown.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMarkdown {
    /// 本次回复的 Markdown 文本内容（可选）。
    pub content: Option<String>,
    /// 自定义模板 ID（可选）。
    pub custom_template_id: Option<String>,
    /// 模板参数（可选）。
    pub params: Option<Vec<MessageMarkdownParam>>,
    /// 键盘（可选）。
    #[serde(skip)]
    pub keyboard: Option<Keyboard>,
}

/// Markdown 模板参数项，表示一个键对应多个值。
///
/// - `key`：参数名。
/// - `values`：参数可以有的多个值（按顺序或作为备选）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMarkdownParam {
    /// 参数名。
    pub key: String,
    /// 参数值列表。
    pub values: Vec<String>,
}

/// 消息按钮键盘对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyboard {
    /// 键盘内容。
    pub content: KeyboardContent,
}

/// 消息按钮键盘内容。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardContent {
    /// 键盘行列表。
    pub rows: Vec<KeyboardRow>,
}

/// 消息按钮键盘中的一行。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardRow {
    /// 当前行中的按钮列表。
    pub buttons: Vec<KeyboardButton>,
}

/// 消息按钮对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// 按钮 ID：在一个 keyboard 消息内设置唯一
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// 按钮渲染数据。
    pub render_data: RenderData,

    /// 按钮行为配置。
    pub action: Action,
}

/// 消息按钮渲染数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderData {
    /// 按钮上的文字
    pub label: String,

    /// 点击后按钮上的文字
    pub visited_label: String,

    /// 按钮样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ButtonStyle>,
}

/// 消息按钮行为配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// 按钮类型
    #[serde(rename = "type")]
    pub action_type: ActionType,

    /// 权限配置
    pub permission: Permission,

    /// 操作相关的数据
    pub data: String,

    /// 指令按钮可用，指令是否带引用回复本消息，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<bool>,

    /// 指令按钮可用，点击按钮后直接自动发送 data，仅单聊可用，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter: Option<bool>,

    /// 指令按钮可用，设置后会忽略 action.enter 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<ActionAnchor>,

    /// 已弃用：可操作点击的次数，默认不限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_limit: Option<i32>,

    /// 已弃用：指令按钮可用，弹出子频道选择器，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_bot_show_channel_list: Option<bool>,

    /// 客户端不支持本 action 的时候，弹出的 toast 文案
    pub unsupport_tips: String,
}

/// 消息按钮权限配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    /// 权限类型
    #[serde(rename = "type")]
    pub permission_type: PermissionType,

    /// 有权限的用户 id 的列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specify_user_ids: Option<Vec<String>>,

    /// 有权限的身份组 id 的列表，仅频道可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specify_role_ids: Option<Vec<String>>,
}

/// 按钮样式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ButtonStyle {
    /// 灰色线框
    GrayOutline = 0,

    /// 蓝色线框
    BlueOutline = 1,
}

/// 按钮行为类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ActionType {
    /// 跳转按钮：http 或小程序客户端识别 scheme
    Jump = 0,

    /// 回调按钮：回调后台接口，data 传给后台
    Callback = 1,

    /// 指令按钮：自动在输入框插入 @bot data
    Command = 2,
}

/// 权限类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PermissionType {
    /// 指定用户可操作
    SpecifyUser = 0,

    /// 仅管理者可操作
    AdminOnly = 1,

    /// 所有人可操作
    Everyone = 2,

    /// 指定身份组可操作，仅频道可用
    SpecifyRole = 3,
}

/// 指令按钮锚点行为
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ActionAnchor {
    /// 点击按钮自动唤起手 Q 选图器
    OpenImagePicker = 1,
}

/// ARK 消息
///
/// https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/type/ark.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArk {
    /// Ark 模板 ID。
    pub template_id: u64,
    /// Ark 模板的键值对。
    pub kv: Vec<MessageArkKv>,
}

/// Ark 模板中的键值对项。
///
/// - `key`：字段名或占位符名。
/// - `value`：对应的字符串值。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArkKv {
    /// 字段名。
    pub key: String,
    /// 字段值。
    pub value: String,
}

///  Embed 消息
///
/// https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/type/embed.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbed {
    /// 卡片标题（可选）。
    pub title: Option<String>,
    /// 卡片提示或描述（可选）。
    pub prompt: Option<String>,
    /// 缩略图（可选）。
    pub thumbnail: Option<MessageEmbedThumbnail>,
    /// 卡片字段（可选）。
    pub fields: Option<Vec<MessageEmbedField>>,
}

/// 嵌入卡片的缩略图信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbedThumbnail {
    /// 缩略图 URL（可选）。
    pub url: Option<String>,
}

/// 嵌入卡片中的单个字段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbedField {
    /// 字段名（可选）。
    pub name: Option<String>,
    /// 字段值（可选）。
    pub value: Option<String>,
}

/// 媒体类型的消息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMedia {
    /// 文件类型（数字编码）。
    pub file_type: u8,
    /// 文件 URL。
    pub url: String,
    /// 是否由服务端发送消息。
    pub srv_send_msg: bool,
    /// 可选的文件数据（例如 base64 字符串）。
    pub file_data: Option<String>,
}

/// PATCH /channels/{channel_id}/messages/{message_id} 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateMessageRequest {
    /// 要回复的消息 ID。
    #[serde(default)]
    pub msg_id: Option<String>,
    /// 要回复的事件 ID。
    #[serde(default)]
    pub event_id: Option<String>,
    /// Markdown 消息对象。
    #[serde(default)]
    pub markdown: Option<MessageMarkdown>,
    /// Keyboard 消息对象。
    #[serde(default)]
    pub keyboard: Option<Keyboard>,
}

/// DELETE 消息接口查询参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteMessageOptions {
    /// 是否隐藏提示小灰条，true 为隐藏，false 为显示，默认 false。
    #[serde(default)]
    pub hidetip: Option<bool>,
}

/// POST /v2/users/{openid}/files 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadMediaRequest {
    /// 媒体类型：1 图片，2 视频，3 语音，4 文件（暂不开放）。
    pub file_type: u8,
    /// 需要发送媒体资源的 URL。
    pub url: String,
    /// 设置 true 会直接发送消息到目标端，且会占用主动消息频次。
    pub srv_send_msg: bool,
    /// 暂未支持的文件数据字段，透传给 OpenAPI。
    #[serde(default)]
    pub file_data: Option<String>,
}

/// POST /v2/users/{openid}/files 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UploadMediaResponse {
    /// 文件 ID。
    #[serde(default)]
    pub file_uuid: Option<String>,
    /// 文件信息，用于发消息接口的 media 字段使用。
    #[serde(default)]
    pub file_info: Option<String>,
    /// 有效期，剩余多少秒到期，0 表示可长期使用。
    #[serde(default)]
    pub ttl: Option<u64>,
    /// 当 srv_send_msg 设置为 true 时返回的消息唯一 ID。
    #[serde(default)]
    pub id: Option<String>,
    /// OpenAPI 返回的未显式建模字段。
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// POST /users/@me/dms 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDmsRequest {
    /// 接收者 id。
    pub recipient_id: String,
    /// 源频道 id。
    pub source_guild_id: String,
}

/// DMS 对象。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dms {
    /// 私信频道对应 guild_id。
    #[serde(default)]
    pub guild_id: Option<String>,
    /// 私信子频道 ID。
    #[serde(default)]
    pub channel_id: Option<String>,
    /// 创建时间。
    #[serde(default)]
    pub create_time: Option<String>,
    /// OpenAPI 返回的未显式建模字段。
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// POST /channels/{channel_id}/audio 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioControlRequest {
    /// 音频地址。
    #[serde(default)]
    pub audio_url: Option<String>,
    /// 状态文本。
    #[serde(default)]
    pub text: Option<String>,
    /// 音频控制状态。
    #[serde(default)]
    pub status: Option<i32>,
}

/// POST /v2/groups/{group_openid}/members 查询参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupMembersQuery {
    /// 每页限制数量 1-500。
    #[serde(default)]
    pub limit: Option<u32>,
    /// 首页输入 0，后续填入返回参数的 next_index。
    #[serde(default)]
    pub start_index: Option<u32>,
}

/// 群成员 openid 对象。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupMember {
    /// 群成员 openid。
    #[serde(default)]
    pub member_openid: Option<String>,
    /// 加群时间戳。
    #[serde(default)]
    pub join_timestamp: Option<u64>,
    /// OpenAPI 返回的未显式建模字段。
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// POST /v2/groups/{group_openid}/members 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupMembersResponse {
    /// openid 列表。
    #[serde(default)]
    pub members: Vec<GroupMember>,
    /// 下一页的拉取标记位。
    #[serde(default)]
    pub next_index: Option<u32>,
    /// OpenAPI 返回的未显式建模字段。
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
