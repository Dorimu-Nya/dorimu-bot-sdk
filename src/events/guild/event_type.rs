use serde::{Deserialize, Serialize};
use super::audio::{AudioOrLiveChannelMemberEvent};
use super::forum::{ OpenForumEvent};
use super::guild::{GuildEvent, ChannelEvent, };
use super::messages::{GuildMessages};
use super::member::{GuildMemberEvent};
/// 频道事件
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum GuildEventType {
    /// 频道内 @ 机器人的消息事件
    AtMessageCreate(GuildMessages),
    /// 撤回频道消息公域事件
    PublicMessageDelete(),
    /// 私信创建事件
    DirectMessageCreate(GuildMessages),
    /// 频道私信删除事件
    DirectMessageDelete(),
    /// 为消息添加表情表态
    MessageReactionAdd,
    /// 为消息删除表情表态
    MessageReactionRemove,
    /// 频道内消息审核通过
    MessageAuditPass(),
    /// 频道内消息审核不通过
    MessageAuditReject(),
    /// 公域论坛事件：用户创建主题
    OpenForumThreadCreate(OpenForumEvent),
    /// 公域论坛事件：用户创建帖子
    OpenForumPostCreate(OpenForumEvent),
    /// 公域论坛事件：用户回复帖子
    OpenForumReplyCreate(OpenForumEvent),
    /// 公域论坛事件：用户更新主题
    OpenForumThreadUpdate(OpenForumEvent),
    /// 公域论坛事件：用户删除帖子
    OpenForumPostDelete(OpenForumEvent),
    /// 公域论坛事件：用户回复被删除
    OpenForumReplyDelete(OpenForumEvent),
    /// 公域论坛事件：用户删除主题
    OpenForumThreadDelete(OpenForumEvent),
    /// 频道创建事件
    GuildCreate(GuildEvent),
    /// 频道信息变更事件
    GuildUpdate(GuildEvent),
    /// 频道删除事件
    GuildDelete(GuildEvent),
    /// 子频道创建事件
    ChannelCreate(ChannelEvent),
    /// 子频道修改事件
    ChannelUpdate(ChannelEvent),
    /// 子频道删除事件
    ChannelDelete(ChannelEvent),
    /// 新成员加入频道事件
    GuildMemberAdd(GuildMemberEvent),
    /// 频道成员离开频道事件
    GuildMemberRemove(GuildMemberEvent),
    /// 频道成员信息更新
    GuildMemberUpdate(GuildMemberEvent),
    /// 音频开始播放事件
    AudioStart(),
    /// 音频播放结束事件
    AudioFinish(),
    /// 机器人上麦事件
    AudioOnMic(),
    /// 机器人下麦事件
    AudioOffMic(),
    // 下面这两个在BOT后台webhook可订阅列表里没看见有，但是文档里的事件列表又有，不是很懂
    /// 音视频/直播子频道成员进事件
    AudioOrLiveChannelMemberEnter(AudioOrLiveChannelMemberEvent),
    /// 音视频/直播子频道成员出事件
    AudioOrLiveChannelMemberExit(AudioOrLiveChannelMemberEvent),
}
