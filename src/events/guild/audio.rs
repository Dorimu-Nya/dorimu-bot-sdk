use serde::{Deserialize, Serialize};

/// 音频/直播子频道成员事件
/// 
/// 触发场景：音频/直播子频道成员进入或退出时产生
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioOrLiveChannelMemberEvent {
    /// 频道ID
    pub guild_id: String,
    /// 子频道ID
    pub channel_id: String,
    #[serde(rename = "type")]
    /// 子频道类型
    pub channel_type: Option<i64>,
    /// 用户ID
    pub user_id: String,
}
