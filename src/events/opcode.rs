use serde_repr::{Deserialize_repr, Serialize_repr};


#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum DispatchOp {
    Dispatch = 0,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum HttpCallbackAckOp {
    HttpCallbackAck = 12,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum WebhookAddressVerifyOp {
    WebhookAddressVerify = 13,
}