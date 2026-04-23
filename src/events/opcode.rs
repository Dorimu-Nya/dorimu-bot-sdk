use serde_repr::{Deserialize_repr, Serialize_repr};

/// 服务端进行消息推送
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum DispatchOp {
    Dispatch = 0,
}

/// 仅用于 http 回调模式的回包，代表机器人收到了平台推送的数据
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum HttpCallbackAckOp {
    HttpCallbackAck = 12,
}

/// 开放平台对机器人服务端进行验证
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum WebhookAddressVerifyOp {
    WebhookAddressVerify = 13,
}
