use crate::events::payload::WebhookPayload;
use crate::handler::{dispatch_event, handle_address_verify};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::{Json, Router};

/// 启动QQBot程序
///
/// example:
/// ```rust
/// use qqbot_sdk_rs::runner::run_application;
///
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
///     run_application().await
/// }
/// ```
/// TODO: 接收项目的参数，如监听端口等
pub async fn run_application() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/webhook", any(webhook_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await
}

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
