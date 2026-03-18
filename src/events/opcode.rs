use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Opcode {
    Dispatch = 0,
    HttpCallbackAck = 12,
    WebhookAddressVerify = 13
}