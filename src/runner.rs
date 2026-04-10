use crate::app::App;
use crate::config::AppConfig;
use crate::events::payload::WebhookPayload;
use axum::routing::any;
use axum::{Json, Router};
use std::sync::Arc;
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

    let webhook_path = config.listening.webhook_path.clone();
    let bind_addr = config.listening.bind_addr.clone();

    let app = Arc::new(App::new(config));
    let router = Router::new().route(
        &webhook_path,
        any({
            let app = Arc::clone(&app);
            move |Json(payload): Json<WebhookPayload>| {
                let app = Arc::clone(&app);
                async move { app.webhook_handler(payload).await }
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

    axum::serve(listener, router).await
}
