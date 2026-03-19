use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::any;
use crate::events::payload::WebhookPayload;
use crate::handler::{dispatch_event, handle_address_verify};

pub async fn run_application() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/webhook", any(webhook_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await
}

async fn webhook_handler(Json(payload): Json<WebhookPayload>) -> impl IntoResponse  {
    println!("{:?}", payload);
    match payload {
        WebhookPayload::Dispatch(payload) => {
            dispatch_event(payload);
            ().into_response()
        },
        WebhookPayload::HttpCallbackAck(_) => ().into_response(),
        WebhookPayload::WebhookAddressVerify(payload) => Json(handle_address_verify(payload.d).unwrap()).into_response()
    }
}