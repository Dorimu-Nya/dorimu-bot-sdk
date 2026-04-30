use serde::{Deserialize, Serialize};

/// 返回 [Guild](model.md#guild) 对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    /// 频道 ID。
    pub id: String,
    /// 频道名称。
    pub name: String,
    /// 频道头像。
    #[serde(default)]
    pub icon: Option<String>,
    /// 频道主创建者的用户 ID。
    #[serde(default)]
    pub owner_id: Option<String>,
    /// 是否是当前用户（机器人）创建。
    #[serde(default)]
    pub owner: Option<bool>,
    /// 加入频道的时间。
    #[serde(default)]
    pub joined_at: Option<String>,
    /// 频道成员数量。
    #[serde(default)]
    pub member_count: Option<u32>,
    /// 频道最大成员数量。
    #[serde(default)]
    pub max_members: Option<u32>,
    /// 频道描述。
    #[serde(default)]
    pub description: Option<String>,
}

/// 返回 [Channel](model.md#channel) 对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    /// 子频道 ID。
    pub id: String,
    /// 频道 ID。
    #[serde(default)]
    pub guild_id: Option<String>,
    /// 子频道名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 子频道类型。
    #[serde(rename = "type")]
    #[serde(default)]
    pub kind: Option<i32>,
    /// 子频道位置。
    #[serde(default)]
    pub position: Option<i32>,
    /// 父子频道 ID。
    #[serde(default)]
    pub parent_id: Option<String>,
    /// 子频道拥有者用户 ID。
    #[serde(default)]
    pub owner_id: Option<String>,
    /// 子频道子类型。
    #[serde(default)]
    pub sub_type: Option<i32>,
    /// 子频道私密类型。
    #[serde(default)]
    pub private_type: Option<i32>,
    /// 子频道发言权限。
    #[serde(default)]
    pub speak_permission: Option<i32>,
    /// 应用类型子频道应用 AppID，仅应用子频道需要该字段。
    #[serde(default)]
    pub application_id: Option<String>,
}

/// 返回 [User](model.md#user) 对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户 ID。
    pub id: String,
    /// 用户名。
    pub username: String,
    /// 用户头像地址。
    #[serde(default)]
    pub avatar: Option<String>,
    /// 是否是机器人账号。
    #[serde(default)]
    pub bot: Option<bool>,
    /// 用户在当前 APPID 下的 openid。
    #[serde(default)]
    pub union_openid: Option<String>,
    /// 用户在当前 APPID 下的 user account。
    #[serde(default)]
    pub union_user_account: Option<String>,
    /// 用户公开标记。
    #[serde(default)]
    pub public_flags: Option<u64>,
    /// 是否系统用户。
    #[serde(default)]
    pub system: Option<bool>,
}

/// 返回 [Member](model.md#member) 成员对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    /// 用户信息。
    pub user: User,
    /// 成员昵称。
    #[serde(default)]
    pub nick: Option<String>,
    /// 成员身份组 ID 列表。
    #[serde(default)]
    pub roles: Vec<String>,
    /// 加入频道时间。
    #[serde(default)]
    pub joined_at: Option<String>,
    /// 是否闭麦。
    #[serde(default)]
    pub deaf: Option<bool>,
    /// 是否被禁言。
    #[serde(default)]
    pub mute: Option<bool>,
    /// 是否待审核。
    #[serde(default)]
    pub pending: Option<bool>,
}

/// 频道身份组对象。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// 身份组 ID。
    pub id: String,
    /// 身份组名称。
    pub name: String,
    /// ARGB 的 HEX 十六进制颜色值转换后的十进制数值。
    pub color: u32,
    /// 在成员列表中单独展示: 0-否, 1-是。
    pub hoist: i32,
    /// 身份组当前成员数量。
    #[serde(default)]
    pub number: Option<u32>,
    /// 身份组成员数量上限。
    #[serde(default)]
    pub member_limit: Option<u32>,
}
