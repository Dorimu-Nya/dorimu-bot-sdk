use serde::{Deserialize, Serialize};
use super::models::{C2cMessage, C2cMsgReceiveEvent, C2cMsgRejectEvent, FriendAddEvent, FriendDelEvent};

/// 单聊事件

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum C2cEventType {
    /// C2C 消息事件
    #[serde(rename = "C2C_MESSAGE_CREATE")]
    C2cMessageCreate(C2cMessage),
    /// C2C 添加好友
    #[serde(rename = "FRIEND_ADD")]
    FriendAdd(FriendAddEvent),
    /// C2C 删除好友
    #[serde(rename = "FRIEND_DEL")]
    FriendDel(FriendDelEvent),
    /// C2C 关闭消息推送
    #[serde(rename = "C2C_MSG_REJECT")]
    C2cMsgReject(C2cMsgRejectEvent),
    /// C2C 打开消息推送
    #[serde(rename = "C2C_MSG_RECEIVE")]
    C2cMsgReceive(C2cMsgReceiveEvent),
}
