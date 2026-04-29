use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// 论坛事件对象
///
/// https://bot.q.qq.com/wiki/develop/api-v2/server-inter/channel/content/forum/model.html#
#[derive(Debug, Clone, Serialize, Deserialize)]
/// 主题事件
///
/// 话题频道内发表的主帖称为主题
///
/// 该事件在话题频道内新发表主题或删除时生产事件中包含该对象
pub struct ForumEventThread {
    /// 频道ID
    pub guild_id: String,
    /// 子频道ID
    pub channel_id: String,
    /// 作者ID
    pub author_id: String,
    /// 主帖内容
    pub thread_info: ForumEventThreadInfo,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// 帖子事件包含的主帖内容相关信息
pub struct ForumEventThreadInfo {
    #[serde(default, alias = "id")]
    /// 主帖ID
    pub thread_id: Option<String>,
    #[serde(default)]
    /// 帖子标题
    pub title: Option<RichTextValue>,
    #[serde(default)]
    /// 帖子内容
    pub content: Option<RichTextValue>,
    #[serde(default)]
    /// 发表时间
    pub date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// 帖子事件
///
/// 话题频道内对主题的评论称为帖子
///
/// 话题频道内对帖子主题评论或删除时生产事件中包含该对象
pub struct ForumEventPost {
    /// 频道ID
    pub guild_id: String,
    /// 子频道ID
    pub channel_id: String,
    /// 作者ID
    pub author_id: String,
    /// 帖子内容
    pub post_info: ForumEventPostInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// 帖子内容信息
///
/// 帖子事件包含的帖子内容信息
pub struct ForumEventPostInfo {
    #[serde(default, alias = "id")]
    /// 帖子ID
    pub post_id: Option<String>,
    #[serde(default)]
    /// 主题ID
    pub thread_id: Option<String>,
    #[serde(default)]
    /// 帖子内容
    pub content: Option<RichTextValue>,
    #[serde(default)]
    /// 评论时间
    pub date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 回复事件
///
/// 话题频道对帖子回复或删除时生产该事件中包含该对象
///
/// 话题频道对帖子回复或删除时生产该事件中包含该对象
pub struct ForumEventReply {
    /// 频道ID
    pub guild_id: String,
    /// 子频道ID
    pub channel_id: String,
    /// 作者ID
    pub author_id: String,
    /// 回复内容
    pub reply_info: ForumEventReplyInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// 回复事件包含的回复内容信息
pub struct ForumEventReplyInfo {
    #[serde(default, alias = "id")]
    /// 回复ID
    pub reply_id: Option<String>,
    #[serde(default)]
    /// 主题ID
    pub thread_id: Option<String>,
    #[serde(default)]
    /// 帖子ID
    pub post_id: Option<String>,
    #[serde(default)]
    /// 回复内容
    pub content: Option<RichTextValue>,
    #[serde(default)]
    /// 回复时间
    pub date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 论坛帖子审核结果事件
pub struct ForumEventAuditResult {
    /// 频道ID
    pub guild_id: String,
    /// 子频道ID
    pub channel_id: String,
    /// 作者ID
    pub author_id: String,
    #[serde(rename = "type")]
    /// 审核类型
    pub kind: ForumEventAuditType,
    /// 审核结果. 0:成功 1:失败
    pub result: Option<i64>,
    /// 错误信息
    pub err_msg: Option<String>,
    /// 主题ID
    pub thread_id: Option<String>,
    /// 帖子ID
    pub post_id: Option<String>,
    /// 回复ID
    pub reply_id: Option<String>,
}

/// 论坛帖子审核类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForumEventAuditType {
    /// 帖子
    PublishTread = 1,
    /// 评论
    PublishPost = 2,
    /// 回复
    PublishReply = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
/// 富文本内容
pub enum RichTextValue {
    /// 普通文本
    Plain(String),
    /// 富文本对象数组
    Objects(Vec<RichObject>),
    /// 富文本结构
    RichText(RichText),
    /// 其他未识别结构
    Other(Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 富文本对象
pub struct RichObject {
    #[serde(rename = "type")]
    /// 富文本类型
    pub kind: RichType,
    #[serde(default)]
    /// 文本信息
    pub text_info: Option<TextInfo>,
    #[serde(default)]
    /// @内容
    pub at_info: Option<AtInfo>,
    #[serde(default)]
    /// 链接信息
    pub url_info: Option<UrlInfo>,
    #[serde(default)]
    /// 表情信息
    pub emoji_info: Option<EmojiInfo>,
    #[serde(default)]
    /// 提到的子频道
    pub channel_info: Option<ChannelInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
/// 富文本对象类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RichType {
    /// 普通消息
    Text = 1,
    /// at 消息
    At = 2,
    /// url消息
    Url = 3,
    /// 表情
    Emoji = 4,
    /// #子频道
    Channel = 5,
    /// 视频
    Video = 10,
    /// 图片
    Image = 11,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextInfo {
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AtInfo {
    #[serde(rename = "type")]
    pub kind: Option<i64>,
    pub user_info: Option<AtUserInfo>,
    pub role_info: Option<AtRoleInfo>,
    pub guild_info: Option<AtGuildInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AtUserInfo {
    pub id: Option<String>,
    pub nick: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AtRoleInfo {
    pub role_id: Option<String>,
    pub name: Option<String>,
    pub color: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AtGuildInfo {
    pub guild_id: Option<String>,
    pub guild_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UrlInfo {
    pub url: Option<String>,
    pub display_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmojiInfo {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub emoji_type: Option<i64>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChannelInfo {
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RichText {
    pub paragraphs: Option<Vec<Paragraph>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Paragraph {
    pub elems: Option<Vec<Elem>>,
    pub props: Option<ParagraphProps>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Elem {
    pub text: Option<TextElem>,
    pub image: Option<ImageElem>,
    pub video: Option<VideoElem>,
    pub url: Option<UrlElem>,
    #[serde(rename = "type")]
    pub elem_type: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextElem {
    pub text: Option<String>,
    pub props: Option<TextProps>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextProps {
    pub font_bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageElem {
    pub third_url: Option<String>,
    pub width_percent: Option<f64>,
    pub plat_image: Option<PlatImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatImage {
    pub url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VideoElem {
    pub third_url: Option<String>,
    pub plat_video: Option<PlatVideo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatVideo {
    pub url: Option<String>,
    pub duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UrlElem {
    pub url: Option<String>,
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParagraphProps {
    pub alignment: Option<i64>,
}
