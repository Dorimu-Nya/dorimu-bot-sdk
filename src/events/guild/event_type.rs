use super::audio::AudioOrLiveChannelMemberEvent;
use super::forum::OpenForumEvent;
use super::guild::{ChannelEvent, GuildEvent};
use super::member::GuildMemberEvent;
use super::messages::GuildMessages;
use serde::{Deserialize, Serialize};
/// 频道事件
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum GuildEventType {
    /// 频道内 @ 机器人的消息事件
    #[serde(rename = "AT_MESSAGE_CREATE")]
    AtMessageCreate(GuildMessages),
    /// 撤回频道消息公域事件
    #[serde(rename = "PUBLIC_MESSAGE_DELETE")]
    PublicMessageDelete(),
    /// 私信创建事件
    #[serde(rename = "DIRECT_MESSAGE_CREATE")]
    DirectMessageCreate(GuildMessages),
    /// 频道私信删除事件
    #[serde(rename = "DIRECT_MESSAGE_DELETE")]
    DirectMessageDelete(),
    /// 为消息添加表情表态
    #[serde(rename = "MESSAGE_REACTION_ADD")]
    MessageReactionAdd,
    /// 为消息删除表情表态
    #[serde(rename = "MESSAGE_REACTION_REMOVE")]
    MessageReactionRemove,
    /// 频道内消息审核通过
    #[serde(rename = "MESSAGE_AUDIT_PASS")]
    MessageAuditPass(),
    /// 频道内消息审核不通过
    #[serde(rename = "MESSAGE_AUDIT_REJECT")]
    MessageAuditReject(),
    /// 公域论坛事件：用户创建主题
    #[serde(rename = "FORUM_THREAD_CREATE")]
    OpenForumThreadCreate(OpenForumEvent),
    /// 公域论坛事件：用户创建帖子
    #[serde(rename = "FORUM_POST_CREATE")]
    OpenForumPostCreate(OpenForumEvent),
    /// 公域论坛事件：用户回复帖子
    #[serde(rename = "FORUM_REPLY_CREATE")]
    OpenForumReplyCreate(OpenForumEvent),
    /// 公域论坛事件：用户更新主题
    #[serde(rename = "FORUM_THREAD_UPDATE")]
    OpenForumThreadUpdate(OpenForumEvent),
    /// 公域论坛事件：用户删除帖子
    #[serde(rename = "FORUM_POST_DELETE")]
    OpenForumPostDelete(OpenForumEvent),
    /// 公域论坛事件：用户回复被删除
    #[serde(rename = "FORUM_REPLY_DELETE")]
    OpenForumReplyDelete(OpenForumEvent),
    /// 公域论坛事件：用户删除主题
    #[serde(rename = "FORUM_THREAD_DELETE")]
    OpenForumThreadDelete(OpenForumEvent),
    /// 频道创建事件
    #[serde(rename = "GUILD_CREATE")]
    GuildCreate(GuildEvent),
    /// 频道信息变更事件
    #[serde(rename = "GUILD_UPDATE")]
    GuildUpdate(GuildEvent),
    /// 频道删除事件
    #[serde(rename = "GUILD_DELETE")]
    GuildDelete(GuildEvent),
    /// 子频道创建事件
    #[serde(rename = "CHANNEL_CREATE")]
    ChannelCreate(ChannelEvent),
    /// 子频道修改事件
    #[serde(rename = "CHANNEL_UPDATE")]
    ChannelUpdate(ChannelEvent),
    /// 子频道删除事件
    #[serde(rename = "CHANNEL_DELETE")]
    ChannelDelete(ChannelEvent),
    /// 新成员加入频道事件
    #[serde(rename = "GUILD_MEMBER_ADD")]
    GuildMemberAdd(GuildMemberEvent),
    /// 频道成员离开频道事件
    #[serde(rename = "GUILD_MEMBER_REMOVE")]
    GuildMemberRemove(GuildMemberEvent),
    /// 频道成员信息更新
    #[serde(rename = "GUILD_MEMBER_UPDATE")]
    GuildMemberUpdate(GuildMemberEvent),
    /// 音频开始播放事件
    #[serde(rename = "AUDIO_START")]
    AudioStart(),
    /// 音频播放结束事件
    #[serde(rename = "AUDIO_FINISH")]
    AudioFinish(),
    /// 机器人上麦事件
    #[serde(rename = "AUDIO_ON_MIC")]
    AudioOnMic(),
    /// 机器人下麦事件
    #[serde(rename = "AUDIO_OFF_MIC")]
    AudioOffMic(),
    // 下面这两个在BOT后台webhook可订阅列表里没看见有，但是文档里的事件列表又有，不是很懂
    /// 音视频/直播子频道成员进事件
    #[serde(rename = "AUDIO_OR_LIVE_CHANNEL_MEMBER_ENTER")]
    AudioOrLiveChannelMemberEnter(AudioOrLiveChannelMemberEvent),
    /// 音视频/直播子频道成员出事件
    #[serde(rename = "AUDIO_OR_LIVE_CHANNEL_MEMBER_EXIT")]
    AudioOrLiveChannelMemberExit(AudioOrLiveChannelMemberEvent),
}
