use crate::{Error, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct EventSchema {
    pub name_path: String,
    pub data_path: Option<String>,
    pub id_path: Option<String>,
    pub timestamp_path: Option<String>,
    pub data_required: bool,
}

impl Default for EventSchema {
    fn default() -> Self {
        Self {
            name_path: "t".to_string(),
            data_path: None,
            id_path: None,
            timestamp_path: None,
            data_required: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventEnvelope<T> {
    pub name: String,
    pub data: T,
    pub id: Option<String>,
    pub timestamp: Option<String>,
    pub raw: Value,
}

impl EventSchema {
    pub fn extract(&self, value: &Value) -> Result<EventEnvelope<Value>> {
        let name = extract_string(value, &self.name_path)?;
        let data = match &self.data_path {
            Some(path) => extract_value(value, path)
                .cloned()
                .or_else(|| if self.data_required { None } else { Some(Value::Null) })
                .ok_or_else(|| Error::Other("event data missing".to_string()))?,
            None => value.clone(),
        };

        Ok(EventEnvelope {
            name,
            data,
            id: self.id_path.as_ref().and_then(|p| extract_string_opt(value, p)),
            timestamp: self
                .timestamp_path
                .as_ref()
                .and_then(|p| extract_string_opt(value, p)),
            raw: value.clone(),
        })
    }

    pub fn extract_t<T: DeserializeOwned>(&self, value: &Value) -> Result<EventEnvelope<T>> {
        let envelope = self.extract(value)?;
        let data = serde_json::from_value(envelope.data).map_err(Error::Json)?;
        Ok(EventEnvelope {
            name: envelope.name,
            data,
            id: envelope.id,
            timestamp: envelope.timestamp,
            raw: envelope.raw,
        })
    }
}

fn extract_string(value: &Value, path: &str) -> Result<String> {
    extract_value(value, path)
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .ok_or(Error::EventNameNotFound)
}

fn extract_string_opt(value: &Value, path: &str) -> Option<String> {
    extract_value(value, path).and_then(|v| v.as_str().map(|s| s.to_string()))
}

fn extract_value<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    if path.starts_with('/') {
        value.pointer(path)
    } else {
        value.get(path)
    }
}
