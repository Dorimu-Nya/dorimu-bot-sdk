use crate::app::App;
use crate::events::payload::WebhookPayload;
use crate::AppConfig;
use axum::routing::any;
use axum::{Json, Router};
use std::sync::Arc;
use tracing::info;

/// 启动QQBot程序
///
/// example:
/// ```rust
/// use qqbot_sdk::{run_application, AppConfig, CredentialConfig};
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
pub async fn run_application_with_router(
    config: AppConfig,
    base_router: Option<Router>,
) -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let webhook_path = config.listening.webhook_path.clone();
    let bind_addr = config.listening.bind_addr.clone();

    let app = Arc::new(App::new(config));
    let base_router = base_router.unwrap_or(Router::new());
    let router = base_router.route(
        &webhook_path,
        any({
            let app = Arc::clone(&app);
            async move |Json(payload): Json<WebhookPayload>| {
                Json(app.webhook_handler(payload).await)
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Listening on {}", bind_addr);
    axum::serve(listener, router).await
}

pub async fn run_application(config: AppConfig) -> std::io::Result<()> {
    run_application_with_router(config, None).await
}
