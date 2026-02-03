# qqbot_sdk

Webhook-first QQ bot SDK for Rust.

## Features
- Webhook server integration (axum)
- Event routing with middleware chain
- Typed event envelope + schema extractor
- Optional handshake hook for webhook verification flows
- Ed25519 signature verification (configurable headers/encoding)
- Extra event types: reactions, interactions, member events, forum events
- HTTP client with retries
- Token manager + OpenAPI client scaffold + guild/channel/member/interaction/reaction modules
- Extra OpenAPI modules (announces/permissions/roles/pins/schedules/forum/mute/message settings/users) with official defaults

## Quick start (webhook)

```rust
use qqbot_sdk::{event_name_field, EventResponse, EventRouter, WebhookApp, WebhookConfig};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let router = EventRouter::new()
        .route_fn("MESSAGE_CREATE", |ctx| async move {
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
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Signature verification

```rust
use qqbot_sdk::{SignatureConfig, SignatureVerifier};

let public_key = vec![0u8; 32]; // replace with your bot public key bytes
let verifier = SignatureVerifier::new(SignatureConfig::new(public_key))?;
```

Or derive it from Bot Secret:

```rust
use qqbot_sdk::SignatureVerifier;

let verifier = SignatureVerifier::from_bot_secret("your_bot_secret")?;
```

Attach it to `WebhookConfig`:

```rust
use qqbot_sdk::WebhookConfig;

let config = WebhookConfig {
    signature: Some(verifier),
    ..Default::default()
};
```

Defaults:
- Signature header: `x-signature-ed25519`
- Timestamp header: `x-signature-timestamp`
- Encoding: auto (hex first, then base64)

Override if your environment differs.

## OpenAPI client scaffold

```rust
use qqbot_sdk::{HttpTokenProvider, OpenApiClient, OpenApiConfig, RetryPolicy, TokenManager};
use std::time::Duration;

let token_provider = HttpTokenProvider::official("app_id", "client_secret");

let token_manager = TokenManager::new(token_provider, Duration::from_secs(120));
let config = OpenApiConfig::official();
let client = OpenApiClient::new(token_manager, config);
```

Env overrides (optional):

```rust
let token_provider = HttpTokenProvider::from_env_or_official("app_id", "client_secret");
let config = OpenApiConfig::from_env_or_official();
```

Supported env vars:
- `QQ_API_BASE_URL`
- `QQ_TOKEN_URL`

## Typed event envelope

```rust
use qqbot_sdk::{EventResponse, EventRouter, TypedEvent, WebhookApp, WebhookConfig};

let router = EventRouter::new().route_fn("MESSAGE_CREATE", move |ctx| {
    async move {
        let event: TypedEvent = ctx.parse_typed_event()?;
        println!("event type: {}", event.event_type.as_str());
        Ok(EventResponse::ok())
    }
});
```

## Webhook handshake hook

```rust
use qqbot_sdk::{webhook_validation_hook, WebhookConfig};

let config = WebhookConfig {
    hook: Some(webhook_validation_hook("your_bot_secret")),
    ..Default::default()
};
```

## OpenAPI modules (path templates)

```rust
use qqbot_sdk::{OpenApi, OpenApiPaths};

let paths = OpenApiPaths::official_defaults();

let api = OpenApi::new(client, paths);
let _ = api.guilds().get("guild_id").await?;
let _ = api.members().list_with("guild_id", None, Some(100)).await?;
let _ = api.interactions().ack("interaction_id", 0).await?;
let _ = api.forums().list_threads("channel_id").await?;
```

## Examples
- `examples/webhook.rs`: minimal webhook router
- `examples/webhook_advanced.rs`: signature + handshake hook + middleware + interaction ack (uses `QQ_BOT_SECRET`, `QQ_APP_ID`, `QQ_CLIENT_SECRET`)
- `examples/webhook_prod.rs`: production-style setup (healthz + graceful shutdown + env config)

## bot_app skeleton
This repo includes a small app scaffold in `bot_app/` for official QQ webhook bots.

Env vars:
- `QQ_APP_ID`, `QQ_CLIENT_SECRET`, `QQ_BOT_SECRET`
- `QQ_HOST`, `QQ_PORT`, `QQ_WEBHOOK_PATH`
- `BOT_CMD_PREFIX` (default `/`)
- `BOT_DATA_PATH` (default `data/kv.json`)
- `BOT_ALLOW_OPENIDS`, `BOT_DENY_OPENIDS` (optional allow/deny lists)

Run (Windows PowerShell):
```
.\scripts\run_bot_app.ps1
```

## Notes
- This SDK is webhook-first. If your payload uses a different event-name field than `t`, use `event_name_pointer` or `event_name_field` to override.
- If you need structured event models or specific API modules (guilds, channels, members), add them atop `OpenApiClient`.
- `OpenApiPaths::official_defaults()` 已补齐官方文档中明确的路径模板，如遇路径变更请自行覆盖。
