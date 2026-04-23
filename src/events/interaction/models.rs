use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 解析后的附加数据（来自 resolved 字段）。
///
/// 包含按钮操作的相关信息：按钮的 data、id、触发用户 id、功能 id、消息 id 等。
pub struct InteractionResolved {
    /// 操作按钮的 data 字段值（在发送消息按钮时设置）
    pub button_data: Option<String>,
    /// 操作按钮的 id 字段值（在发送消息按钮时设置）
    pub button_id: Option<String>,
    /// 操作的用户 userid，仅频道场景提供该字段
    pub user_id: Option<String>,
    /// 操作按钮的 feature_id，仅自定义菜单提供该字段（在管理端设置）
    pub feature_id: Option<String>,
    /// 操作的消息 id，目前仅频道场景提供该字段
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionData {
    #[serde(default, alias = "resoloved")]
    /// 解析后的按钮/菜单数据
    pub resolved: Option<InteractionResolved>,
}

/// 点击回调按钮
///
/// 触发场景: 用户点击了消息体的回调按钮
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    /// 平台方事件 ID，可以用于被动消息发送
    pub id: String,
    #[serde(rename = "type")]
    /// 消息类型：11 表示消息按钮，12 表示单聊快捷菜单
    pub kind: Option<i64>,
    /// 事件发生的场景：c2c、group、guild
    pub scene: Option<String>,
    /// 聊天类型：0 频道场景，1 群聊场景，2 单聊场景
    pub chat_type: Option<i64>,
    /// 触发时间，RFC 3339 格式
    pub timestamp: Option<String>,
    /// 频道的 openid，仅在频道场景提供该字段
    pub guild_id: Option<String>,
    /// 文字子频道的 openid，仅在频道场景提供该字段
    pub channel_id: Option<String>,
    /// 单聊按钮触发的用户 openid，仅在单聊场景提供该字段
    pub user_openid: Option<String>,
    /// 群的 openid，仅在群聊场景提供该字段
    pub group_openid: Option<String>,
    /// 按钮触发用户在群聊中的成员 openid，仅在群聊场景提供该字段
    pub group_member_openid: Option<String>,
    /// 请求携带的交互数据，包含 type 与 resolved 等字段
    pub data: Option<InteractionData>,
    /// 版本号，默认 1
    pub version: Option<i64>,
}
