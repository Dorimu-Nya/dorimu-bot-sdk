use super::event_type::EventType;
use super::opcode::{DispatchOp, HttpCallbackAckOp, WebhookAddressVerifyOp};
use crate::events::validation::ValidationRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookPayload {
    // opcode=0
    /// 服务端进行消息推送
    Dispatch(DispatchPayload),
    // opcode = 12
    /// http 回调模式的回包
    HttpCallbackAck(HttpCallbackAckPayload),
    // opcode=13
    /// 开放平台对机器人服务端进行验证
    WebhookAddressVerify(WebhookAddressVerifyPayload),
}

/// opcode为0时，服务端进行消息推送的消息对象
#[derive(Debug, Serialize, Deserialize)]
pub struct DispatchPayload {
    /// 事件id
    pub id: Option<String>,
    ///  下行消息都会有一个序列号，标识消息的唯一性，客户端需要再发送心跳的时候，携带客户端收到的最新的s
    pub s: Option<u64>,
    /// 代表事件类型
    pub op: DispatchOp,
    /// 代表事件内容，不同事件类型的事件内容格式都不同，请注意识别。主要用在op为 0 Dispatch 的时候
    #[serde(flatten)]
    pub event: EventType,
}

#[allow(dead_code)] // 未来实现全部事件处理（或拦截器）的时候用
pub trait FromDispatchPayload<'a>: Sized {
    fn from(req: &'a DispatchPayload) -> Self;
}

impl<'a> FromDispatchPayload<'a> for &'a DispatchPayload {
    fn from(req: &'a DispatchPayload) -> Self {
        req
    }
}

/// opcode为12时，http 回调模式的回包
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpCallbackAckPayload {
    pub id: Option<String>,
    pub s: Option<u64>,
    pub op: HttpCallbackAckOp,
}

/// opcode为13时，开放平台对机器人服务端进行验证时的消息对象
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookAddressVerifyPayload {
    pub id: Option<String>,
    pub s: Option<u64>,
    pub op: WebhookAddressVerifyOp,
    pub d: ValidationRequest,
}
