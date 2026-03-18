use serde::{Deserialize, Serialize};

/// 群事件
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum GroupEventType {
    /// 群消息事件 AT 事件
    GroupAtMessageCreate,
    /// 群添加机器人
    GroupAddRobot,
    /// 群移除机器人
    GroupDelRobot,
    /// 群开启消息推送
    GroupMsgReceive,
    /// 群关闭消息推送
    GroupMsgReject,
    /// 消息授权状态变更
    SubscribeMessageStatus,
}
