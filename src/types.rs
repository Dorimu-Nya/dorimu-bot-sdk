use bytes::Bytes;
use http::{HeaderMap, StatusCode};
use serde_json::Value;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct EventContext {
    pub event_name: Option<String>,
    pub payload: Value,
    pub raw_body: Bytes,
    pub headers: HeaderMap,
    pub received_at: SystemTime,
}

impl EventContext {
    pub fn parse_event(&self, schema: &crate::event::EventSchema) -> crate::Result<crate::event::EventEnvelope<Value>> {
        schema.extract(&self.payload)
    }

    pub fn parse_event_t<T: serde::de::DeserializeOwned>(
        &self,
        schema: &crate::event::EventSchema,
    ) -> crate::Result<crate::event::EventEnvelope<T>> {
        schema.extract_t(&self.payload)
    }

    pub fn parse_typed_event(&self) -> crate::Result<crate::events::TypedEvent> {
        crate::events::TypedEvent::from_value(&self.payload)
    }
}

#[derive(Debug, Clone)]
pub struct EventResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Bytes,
}

impl EventResponse {
    pub fn empty(status: StatusCode) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    pub fn ok() -> Self {
        Self::empty(StatusCode::OK)
    }

    pub fn json(value: &Value) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(http::header::CONTENT_TYPE, http::HeaderValue::from_static("application/json"));
        let body = Bytes::from(serde_json::to_vec(value).unwrap_or_default());
        Self {
            status: StatusCode::OK,
            headers,
            body,
        }
    }

    pub fn with_status_json(status: StatusCode, value: &Value) -> Self {
        let mut resp = Self::json(value);
        resp.status = status;
        resp
    }
}
