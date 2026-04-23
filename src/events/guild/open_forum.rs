use serde::{Deserialize, Serialize};
// https://bot.q.qq.com/wiki/develop/api-v2/server-inter/channel/content/forum/open_forum.html
#[derive(Debug, Clone, Serialize, Deserialize)]
/// 开放论坛事件
///
/// 用户在话题子频道内发帖、评论、回复评论时产生该事件
pub struct OpenForumEvent {
    /// 频道ID
    pub guild_id: String,
    /// 子频道ID
    pub channel_id: String,
    /// 作者ID
    pub author_id: String,
}