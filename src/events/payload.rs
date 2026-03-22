use super::event_type::EventType;
use super::opcode::{DispatchOp, HttpCallbackAckOp, WebhookAddressVerifyOp};
use crate::events::validation::ValidationRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookPayload {
    Dispatch(DispatchPayload),
    HttpCallbackAck(HttpCallbackAckPayload),
    WebhookAddressVerify(WebhookAddressVerifyPayload),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DispatchPayload {
    pub id: Option<String>,
    pub s: Option<u64>,
    pub op: DispatchOp,
    #[serde(flatten)]
    pub event: EventType,
}

pub trait FromDispatchPayload<'a>: Sized {
    fn from(req: &'a DispatchPayload) -> Self;
}

impl<'a> FromDispatchPayload<'a> for &'a DispatchPayload {
    fn from(req: &'a DispatchPayload) -> Self {
        req
    }
}




#[derive(Debug, Serialize, Deserialize)]
pub struct HttpCallbackAckPayload {
    pub id: Option<String>,
    pub s: Option<u64>,
    pub op: HttpCallbackAckOp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookAddressVerifyPayload {
    pub id: Option<String>,
    pub s: Option<u64>,
    pub op: WebhookAddressVerifyOp,
    pub d: ValidationRequest,
}
