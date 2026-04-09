use crate::events::payload::WebhookPayload;
use crate::handler::{dispatch_event, handle_address_verify};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::{Json, Router};
use crate::config::AppConfig;
use std::sync::OnceLock;

static GLOBAL_CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// 启动QQBot程序
///
/// example:
/// ```rust
/// use qqbot_sdk_rs::runner::run_application;
///
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
///     let config = AppConfig {
///         credential: CredentialConfig {
///             app_id: "YOUR APP ID".to_string(),
///             secret: "YOUR SECRET".to_string(),
///         },
///         ..Default::default()
///     };
///     run_application(config).await
/// }
/// ```
pub async fn run_application(config: AppConfig) -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    
    let app = Router::new().route(&config.listening.webhook_path, any(webhook_handler));

    let listener = tokio::net::TcpListener::bind(&config.listening.bind_addr).await?;
    
    GLOBAL_CONFIG.set(config).ok();
    axum::serve(listener, app).await
}

// TODO: 存储像openapi客户端等其他实例，考虑要不要把这些处理方法重构成到一个实体对象里

// webhook的第一层的对t字段的处理
async fn webhook_handler(Json(payload): Json<WebhookPayload>) -> impl IntoResponse {
    match payload {
        WebhookPayload::Dispatch(payload) => {
            dispatch_event(payload).await;
            ().into_response()
        }
        WebhookPayload::HttpCallbackAck(_) => ().into_response(),
        WebhookPayload::WebhookAddressVerify(payload) => {
            Json(handle_address_verify(payload.d).unwrap()).into_response()
        }
    }
}
