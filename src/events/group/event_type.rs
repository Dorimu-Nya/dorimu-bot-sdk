use serde::{Deserialize, Serialize};
use super::models::{GroupAddRobotEvent, GroupAtMessage, GroupDelRobotEvent, GroupMsgRejectEvent};

/// 群事件
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum GroupEventType {
    /// 群消息事件 AT 事件
    GroupAtMessageCreate(GroupAtMessage),
    /// 群添加机器人
    GroupAddRobot(GroupAddRobotEvent),
    /// 群移除机器人
    GroupDelRobot(GroupDelRobotEvent),
    /// 群开启消息推送
    GroupMsgReceive(GroupAddRobotEvent),
    /// 群关闭消息推送
    GroupMsgReject(GroupMsgRejectEvent),
    /// 消息授权状态变更
    SubscribeMessageStatus,
}
