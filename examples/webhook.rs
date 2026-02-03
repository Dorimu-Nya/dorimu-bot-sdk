use qqbot_sdk::{event_name_field, EventResponse, EventRouter, WebhookApp, WebhookConfig};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let router = EventRouter::new()
        .route_fn("event.name", |ctx| async move {
            println!("event: {:?}", ctx.event_name);
            Ok(EventResponse::ok())
        })
        .default_handler_fn(|_ctx| async move { Ok(EventResponse::ok()) });

    let config = WebhookConfig {
        path: "/webhook".to_string(),
        event_name_extractor: event_name_field("t"),
        ..Default::default()
    };

    let app = WebhookApp::new(router, config).into_router();

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
