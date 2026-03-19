use serde::{Deserialize, Serialize};
use super::models::{C2cMessage, C2cMsgReceiveEvent, C2cMsgRejectEvent, FriendAddEvent, FriendDelEvent};

/// 单聊事件

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum C2cEventType {
    /// C2C 消息事件
    C2cMessageCreate(C2cMessage),
    /// C2C 添加好友
    FriendAdd(FriendAddEvent),
    /// C2C 删除好友
    FriendDel(FriendDelEvent),
    /// C2C 关闭消息推送
    C2cMsgReject(C2cMsgRejectEvent),
    /// C2C 打开消息推送
    C2cMsgReceive(C2cMsgReceiveEvent),
}
