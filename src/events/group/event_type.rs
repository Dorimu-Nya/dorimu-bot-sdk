use super::models::{GroupAddRobotEvent, GroupAtMessage, GroupDelRobotEvent, GroupMsgRejectEvent};
use serde::{Deserialize, Serialize};

/// 群事件
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum GroupEventType {
    /// 群消息事件 AT 事件
    #[serde(rename = "GROUP_AT_MESSAGE_CREATE")]
    GroupAtMessageCreate(GroupAtMessage),
    /// 群添加机器人
    #[serde(rename = "GROUP_AT_ROBOT")]
    GroupAddRobot(GroupAddRobotEvent),
    /// 群移除机器人
    #[serde(rename = "GROUP_DEL_ROBOT")]
    GroupDelRobot(GroupDelRobotEvent),
    /// 群开启消息推送
    #[serde(rename = "GROUP_MSG_RECEIVE")]
    GroupMsgReceive(GroupAddRobotEvent),
    /// 群关闭消息推送
    #[serde(rename = "GROUP_MSG_REJECT")]
    GroupMsgReject(GroupMsgRejectEvent),
    /// 消息授权状态变更
    #[serde(rename = "SUBSCRIBE_MESSAGE_STATUS")]
    SubscribeMessageStatus,
}
