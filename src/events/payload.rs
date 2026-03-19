use serde::{Deserialize, Serialize};
use crate::events::opcode::Opcode;
use crate::EventType;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub id: Option<String>,
    /// OpCode
    pub op: Opcode,
    pub s: Option<u64>,
    #[serde(flatten)]
    pub event: EventType,
}
