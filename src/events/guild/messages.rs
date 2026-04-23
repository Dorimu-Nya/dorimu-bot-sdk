use super::super::common::Attachment;
use super::member::{Member, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 频道消息
/// 触发场景：频道内消息相关事件
pub struct GuildMessages {
    /// 消息ID
    pub id: String,
    /// 子频道ID
    pub channel_id: Option<String>,
    /// 频道ID
    pub guild_id: Option<String>,
    /// 消息内容
    pub content: Option<String>,
    /// 消息生产时间（RFC3339）
    pub timestamp: Option<String>,
    /// 作者信息
    pub author: Option<User>,
    /// 成员信息
    pub member: Option<Member>,
    /// 富媒体文件附件，文件类型："图片，语音，视频，文件"
    pub attachments: Option<Vec<Attachment>>,
    /// 消息序号
    pub seq: Option<u64>,
    /// 子频道内消息序号
    pub seq_in_channel: Option<u64>,
}
