use crate::{EventContext, EventResponse, EventRouter, Result, SignatureVerifier};
use axum::{
    body::Bytes as AxumBytes,
    extract::State,
    response::IntoResponse,
    routing::post,
    Router,
};
use http::{HeaderMap, StatusCode};
use serde_json::Value;
use std::{sync::Arc, time::SystemTime};

pub type EventNameExtractor = Arc<dyn Fn(&Value) -> Option<String> + Send + Sync>;
pub type WebhookHook = Arc<dyn Fn(&HeaderMap, &[u8], &Value) -> Result<Option<EventResponse>> + Send + Sync>;
pub type RawLogger = Arc<dyn Fn(&HeaderMap, &[u8]) + Send + Sync>;

pub fn event_name_field(field: &'static str) -> EventNameExtractor {
    Arc::new(move |value: &Value| value.get(field).and_then(|v| v.as_str()).map(|s| s.to_string()))
}

pub fn event_name_pointer(pointer: &'static str) -> EventNameExtractor {
    Arc::new(move |value: &Value| value.pointer(pointer).and_then(|v| v.as_str()).map(|s| s.to_string()))
}

pub fn event_name_any(fields: &[&str]) -> EventNameExtractor {
    let keys = fields.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    Arc::new(move |value: &Value| {
        for key in &keys {
            if let Some(name) = value.get(key).and_then(|v| v.as_str()) {
                return Some(name.to_string());
            }
        }
        None
    })
}

pub fn event_name_pointer_any(pointers: &[&str]) -> EventNameExtractor {
    let keys = pointers.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    Arc::new(move |value: &Value| {
        for key in &keys {
            if let Some(name) = value.pointer(key).and_then(|v| v.as_str()) {
                return Some(name.to_string());
            }
        }
        None
    })
}

pub fn webhook_validation_hook(bot_secret: impl Into<String>) -> WebhookHook {
    let secret = bot_secret.into();
    Arc::new(move |_headers, _raw, payload| {
        let op = payload.get("op").and_then(|v| v.as_i64());
        if op != Some(13) {
            return Ok(None);
        }
        let data = payload
            .get("d")
            .ok_or_else(|| crate::Error::Other("missing validation data".to_string()))?;
        let req: crate::ValidationRequest = serde_json::from_value(data.clone()).map_err(crate::Error::Json)?;
        let signature = crate::signature::sign_webhook_validation(&secret, &req.event_ts, &req.plain_token)?;
        let resp = crate::ValidationResponse {
            plain_token: req.plain_token,
            signature,
        };
        Ok(Some(EventResponse::json(
            &serde_json::to_value(resp).map_err(crate::Error::Json)?,
        )))
    })
}
#[derive(Clone)]
pub struct WebhookConfig {
    pub path: String,
    pub signature: Option<SignatureVerifier>,
    pub max_body_bytes: usize,
    pub event_name_extractor: EventNameExtractor,
    pub hook: Option<WebhookHook>,
    pub raw_logger: Option<RawLogger>,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            path: "/".to_string(),
            signature: None,
            max_body_bytes: 256 * 1024,
            event_name_extractor: event_name_field("t"),
            hook: None,
            raw_logger: None,
        }
    }
}

#[derive(Clone)]
pub struct WebhookApp {
    router: EventRouter,
    config: WebhookConfig,
}

impl WebhookApp {
    pub fn new(router: EventRouter, config: WebhookConfig) -> Self {
        Self { router, config }
    }

    pub fn into_router(self) -> Router {
        let state = Arc::new(self);
        let max_body_bytes = state.config.max_body_bytes;
        Router::new()
            .route(&state.config.path, post(handle_webhook))
            .with_state(state)
            .layer(axum::extract::DefaultBodyLimit::max(max_body_bytes))
    }
}

async fn handle_webhook(
    State(app): State<Arc<WebhookApp>>,
    headers: HeaderMap,
    body: AxumBytes,
) -> impl IntoResponse {
    let result = handle_inner(app, headers, body).await;
    match result {
        Ok(resp) => resp.into_response(),
        Err(err) => {
            let status = match err {
                crate::Error::InvalidSignature | crate::Error::MissingHeader(_) => StatusCode::UNAUTHORIZED,
                crate::Error::Json(_) | crate::Error::InvalidHeader(_) | crate::Error::EventNameNotFound => {
                    StatusCode::BAD_REQUEST
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, err.to_string()).into_response()
        }
    }
}

async fn handle_inner(
    app: Arc<WebhookApp>,
    headers: HeaderMap,
    body: AxumBytes,
) -> Result<EventResponse> {
    if let Some(logger) = &app.config.raw_logger {
        (logger)(&headers, &body);
    }

    if let Some(verifier) = &app.config.signature {
        verifier.verify(&headers, &body)?;
    }

    let payload: Value = serde_json::from_slice(&body)?;
    if let Some(hook) = &app.config.hook {
        if let Some(resp) = (hook)(&headers, &body, &payload)? {
            return Ok(resp);
        }
    }
    let event_name = (app.config.event_name_extractor)(&payload);

    let ctx = EventContext {
        event_name,
        payload,
        raw_body: body.into(),
        headers,
        received_at: SystemTime::now(),
    };

    app.router.dispatch(ctx).await
}

impl IntoResponse for EventResponse {
    fn into_response(self) -> axum::response::Response {
        let mut response = (self.status, self.body).into_response();
        let headers = response.headers_mut();
        for (k, v) in self.headers.iter() {
            headers.insert(k, v.clone());
        }
        response
    }
}
