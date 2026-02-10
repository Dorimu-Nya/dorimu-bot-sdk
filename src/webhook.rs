use crate::{EventContext, EventResponse, EventRouter, Result, SignatureVerifier};
use axum::{
    body::Bytes as AxumBytes, extract::State, response::IntoResponse, routing::post, Router,
};
use http::{HeaderMap, StatusCode};
use serde_json::Value;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::SystemTime,
};
use tracing::{error, warn};

/// 从 JSON 中提取事件名的函数类型。
pub type EventNameExtractor = Arc<dyn Fn(&Value) -> Option<String> + Send + Sync>;
/// webhook 预处理钩子类型，返回 `Some` 时将直接作为响应返回。
pub type WebhookHook =
    Arc<dyn Fn(&HeaderMap, &[u8], &Value) -> Result<Option<EventResponse>> + Send + Sync>;
/// 原始请求日志回调类型。
pub type RawLogger = Arc<dyn Fn(&HeaderMap, &[u8]) + Send + Sync>;

// 防止在 monitor 模式且未配置 verifier 时日志刷屏。
static MISSING_SIGNATURE_VERIFIER_WARNED: AtomicBool = AtomicBool::new(false);

/// webhook 签名校验模式。
#[derive(Debug, Clone)]
pub enum SignatureVerificationMode {
    /// 关闭签名校验。
    Off,
    /// 只记录签名失败日志，不拦截请求（灰度阶段推荐）。
    Monitor,
    /// 强制签名校验，失败直接拒绝。
    Enforce,
}

/// 通过顶层字段提取事件名。
pub fn event_name_field(field: &'static str) -> EventNameExtractor {
    Arc::new(move |value: &Value| {
        value
            .get(field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    })
}

/// 通过 JSON Pointer 提取事件名。
pub fn event_name_pointer(pointer: &'static str) -> EventNameExtractor {
    Arc::new(move |value: &Value| {
        value
            .pointer(pointer)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    })
}

/// 按字段优先级依次提取事件名。
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

/// 按 JSON Pointer 优先级依次提取事件名。
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

/// 生成官方验证回调（`op=13`）处理钩子。
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
        let req: crate::ValidationRequest =
            serde_json::from_value(data.clone()).map_err(crate::Error::Json)?;
        let signature =
            crate::signature::sign_webhook_validation(&secret, &req.event_ts, &req.plain_token)?;
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
    /// webhook 路由路径。
    pub path: String,
    /// 签名验证器。
    pub signature: Option<SignatureVerifier>,
    /// 签名校验模式（支持灰度）。
    pub signature_verification: SignatureVerificationMode,
    /// 最大请求体大小。
    pub max_body_bytes: usize,
    /// 事件名提取器。
    pub event_name_extractor: EventNameExtractor,
    /// 业务前置钩子（验证回调等）。
    pub hook: Option<WebhookHook>,
    /// 原始请求日志回调。
    pub raw_logger: Option<RawLogger>,
    /// 是否对外暴露详细错误（默认关闭，避免泄露内部信息）。
    pub expose_error_details: bool,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            path: "/".to_string(),
            signature: None,
            // 默认监控模式，保证旧接入在未开启验签时也能平滑运行。
            signature_verification: SignatureVerificationMode::Monitor,
            max_body_bytes: 256 * 1024,
            event_name_extractor: event_name_field("t"),
            hook: None,
            raw_logger: None,
            // 默认返回脱敏错误文本。
            expose_error_details: false,
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
    // 先提取配置字段，避免分析器对 `app` 生命周期产生误判。
    let expose_error_details = app.config.expose_error_details;
    let result = handle_inner(app.as_ref(), headers, body).await;
    match result {
        Ok(resp) => resp.into_response(),
        Err(err) => {
            // 详细错误写入日志，HTTP 响应默认脱敏。
            error!(error = ?err, "webhook request failed");
            let status = match err {
                crate::Error::InvalidSignature
                | crate::Error::MissingHeader(_)
                | crate::Error::InvalidTimestamp(_) => StatusCode::UNAUTHORIZED,
                crate::Error::Json(_)
                | crate::Error::InvalidHeader(_)
                | crate::Error::EventNameNotFound => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            let body = if expose_error_details {
                err.to_string()
            } else {
                // 对外统一错误文案，减少内部实现细节暴露。
                match status {
                    StatusCode::UNAUTHORIZED => "unauthorized".to_string(),
                    StatusCode::BAD_REQUEST => "bad request".to_string(),
                    _ => "internal server error".to_string(),
                }
            };
            (status, body).into_response()
        }
    }
}

async fn handle_inner(
    app: &WebhookApp,
    headers: HeaderMap,
    body: AxumBytes,
) -> Result<EventResponse> {
    if let Some(logger) = &app.config.raw_logger {
        // 可选原始报文日志，便于联调。
        (logger)(&headers, &body);
    }

    // hook 优先于签名验证执行：QQ 平台的验证回调（op=13）在首次配置
    // webhook 时可能不携带有效签名，必须在签名校验之前处理。
    let payload: Value = serde_json::from_slice(&body)?;
    if let Some(hook) = &app.config.hook {
        if let Some(resp) = (hook)(&headers, &body, &payload)? {
            return Ok(resp);
        }
    }

    match app.config.signature_verification {
        SignatureVerificationMode::Off => {}
        SignatureVerificationMode::Monitor => {
            if let Some(verifier) = &app.config.signature {
                if let Err(err) = verifier.verify(&headers, &body) {
                    // 监控模式仅记录，不阻断业务处理。
                    warn!(error = ?err, "signature verification failed in monitor mode");
                }
            } else if !MISSING_SIGNATURE_VERIFIER_WARNED.swap(true, Ordering::Relaxed) {
                warn!("signature verification is in monitor mode but verifier is not configured");
            }
        }
        SignatureVerificationMode::Enforce => {
            // 强制模式必须提供 verifier。
            let verifier = app.config.signature.as_ref().ok_or_else(|| {
                crate::Error::Other("signature verifier is required in enforce mode".to_string())
            })?;
            verifier.verify(&headers, &body)?;
        }
    }
    let event_name = (app.config.event_name_extractor)(&payload);

    let ctx = EventContext {
        event_name,
        payload,
        raw_body: body,
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
