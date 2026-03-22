use super::models::Interaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum InteractionEventType {
    InteractionCreate(Interaction),
}
