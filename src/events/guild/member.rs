use serde::{Deserialize, Serialize};

/// 频道成员中的用户信息

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 频道成员中的用户信息
pub struct User {
    /// 用户ID
    pub id: Option<String>,
    /// 用户名
    pub username: Option<String>,
    /// 用户头像
    pub avatar: Option<String>,
    /// 是否为机器人
    pub bot: Option<bool>,
    /// 公开标识
    pub public_flags: Option<i64>,
    /// 是否为系统用户
    pub system: Option<bool>,
    /// 联合 openid
    pub union_openid: Option<String>,
    /// 联合用户账号
    pub union_user_account: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// 频道成员信息
pub struct Member {
    /// 加入时间
    pub joined_at: Option<String>,
    /// 角色列表
    pub roles: Option<Vec<String>>,
    #[serde(default)]
    /// 昵称
    pub nick: Option<String>,
    #[serde(default)]
    /// 是否被禁言
    pub deaf: Option<bool>,
    #[serde(default)]
    /// 是否被静音
    pub mute: Option<bool>,
    #[serde(default)]
    /// 是否待审核
    pub pending: Option<bool>,
    #[serde(default)]
    /// 用户信息
    pub user: Option<User>,
    #[serde(default)]
    /// 频道ID
    pub guild_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 频道成员事件
pub struct GuildMemberEvent {
    #[serde(flatten)]
    /// 成员信息
    pub member: Member,
    /// 操作人 ID
    pub op_user_id: Option<String>,
}
