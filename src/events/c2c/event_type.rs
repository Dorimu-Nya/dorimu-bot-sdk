use serde::{Deserialize, Serialize};

/// 单聊事件

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum C2cEventType {
    /// C2C 消息事件
    C2cMessageCreate,
    /// C2C 添加好友
    FriendAdd,
    /// C2C 删除好友
    FriendDel,
    /// C2C 关闭消息推送
    C2cMsgReject,
    /// C2C 打开消息推送
    C2cMsgReceive,
}
