use serde::{Deserialize, Serialize};
use super::models::Interaction;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum InteractionEventType {
    InteractionCreate(Interaction),
}