use super::models::Interaction;
use serde::{Deserialize, Serialize};

/// 互动事件
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum InteractionEventType {
    /// 创建互动事件, 用户点击了消息体的回调按钮触发
    #[serde(rename = "INTERACTION_CREATE")]
    InteractionCreate(Interaction),
}
