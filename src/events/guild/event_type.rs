use serde::{Deserialize, Serialize};

/// 频道事件
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum GuildEventType {
    /// 频道内 @ 机器人的消息事件
    AtMessageCreate,
    /// 撤回频道消息公域事件
    PublicMessageDelete,
    /// 私信创建事件
    DirectMessageCreate,
    /// 频道私信删除事件
    DirectMessageDelete,
    /// 为消息添加表情表态
    MessageReactionAdd,
    /// 为消息删除表情表态
    MessageReactionRemove,
    /// 频道内消息审核通过
    MessageAuditPass,
    /// 频道内消息审核不通过
    MessageAuditReject,
    /// 公域论坛事件：用户创建主题
    OpenForumThreadCreate,
    /// 公域论坛事件：用户创建帖子
    OpenForumPostCreate,
    /// 公域论坛事件：用户回复帖子
    OpenForumReplyCreate,
    /// 公域论坛事件：用户更新主题
    OpenForumThreadUpdate,
    /// 公域论坛事件：用户删除帖子
    OpenForumPostDelete,
    /// 公域论坛事件：用户回复被删除
    OpenForumReplyDelete,
    /// 公域论坛事件：用户删除主题
    OpenForumThreadDelete,
    /// 频道创建事件
    GuildCreate,
    /// 频道信息变更事件
    GuildUpdate,
    /// 频道删除事件
    GuildDelete,
    /// 子频道创建事件
    ChannelCreate,
    /// 子频道修改事件
    ChannelUpdate,
    /// 子频道删除事件
    ChannelDelete,
    /// 新成员加入频道事件
    GuildMemberAdd,
    /// 频道成员离开频道事件
    GuildMemberRemove,
    /// 频道成员信息更新
    GuildMemberUpdate,
    /// 音频开始播放事件
    AudioStart,
    /// 音频播放结束事件
    AudioFinish,
    /// 机器人上麦事件
    AudioOnMic,
    /// 机器人下麦事件
    AudioOffMic,
}
