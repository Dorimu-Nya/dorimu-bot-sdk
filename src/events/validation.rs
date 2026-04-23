use serde::{Deserialize, Serialize};

/// 接收开放平台对机器人服务端进行验证的消息体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRequest {
    /// 需要计算签名的字符串
    pub plain_token: String,
    /// 计算签名使用时间戳
    pub event_ts: String,
}

/// 返回开放平台对机器人服务端进行验证的消息体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResponse {
    /// 需要计算签名的字符串
    pub plain_token: String,
    /// 签名
    pub signature: String,
}
