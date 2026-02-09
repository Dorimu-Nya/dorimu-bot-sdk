use async_trait::async_trait;
use axum::{routing::get, Router};
use qqbot_sdk::{
    event_name_field, webhook_validation_hook, EventContext, EventData, EventResponse, EventRouter,
    HttpTokenProvider, Middleware, Next, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths,
    RawLogger, ReplayProtectionMode, SignatureConfig, SignatureVerificationMode, SignatureVerifier,
    TokenManager, WebhookApp, WebhookConfig,
};
use serde_json::json;
use std::{
    collections::HashMap,
    env,
    io::Write,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct Config {
    host: String,
    port: u16,
    webhook_path: String,
    bot_secret: String,
    app_id: String,
    client_secret: String,
}

impl Config {
    fn from_env() -> Self {
        let host = env::var("QQ_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("QQ_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(3000);
        let webhook_path = env::var("QQ_WEBHOOK_PATH").unwrap_or_else(|_| "/webhook".to_string());
        let bot_secret = env::var("QQ_BOT_SECRET").expect("QQ_BOT_SECRET missing");
        let app_id = env::var("QQ_APP_ID").expect("QQ_APP_ID missing");
        let client_secret = env::var("QQ_CLIENT_SECRET").expect("QQ_CLIENT_SECRET missing");

        Self {
            host,
            port,
            webhook_path,
            bot_secret,
            app_id,
            client_secret,
        }
    }
}

struct LoggingMiddleware;

#[async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(&self, ctx: EventContext, next: Next) -> qqbot_sdk::Result<EventResponse> {
        tracing::info!(event = ?ctx.event_name, "webhook received");
        next.run(ctx).await
    }
}

async fn healthz() -> &'static str {
    "ok"
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received");
}

#[derive(Clone)]
struct Deduper {
    store: Arc<Mutex<HashMap<String, Instant>>>,
    ttl: Duration,
    cap: usize,
}

impl Deduper {
    fn from_env() -> Self {
        let ttl = env::var("QQ_C2C_DEDUP_TTL_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(60);
        let cap = env::var("QQ_C2C_DEDUP_CAP")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(2000);
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
            ttl: Duration::from_secs(ttl),
            cap,
        }
    }

    async fn is_duplicate(&self, key: &str) -> bool {
        if self.cap == 0 {
            return false;
        }
        let now = Instant::now();
        let mut guard = self.store.lock().await;
        guard.retain(|_, ts| now.duration_since(*ts) <= self.ttl);
        if guard.contains_key(key) {
            return true;
        }
        if guard.len() >= self.cap {
            if let Some(oldest_key) = guard
                .iter()
                .min_by_key(|(_, ts)| *ts)
                .map(|(k, _)| k.clone())
            {
                guard.remove(&oldest_key);
            }
        }
        guard.insert(key.to_string(), now);
        false
    }
}

fn parse_keywords(raw: &str) -> Vec<String> {
    raw.split(|c: char| c == ',' || c == ';' || c.is_whitespace())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}

fn matches_keywords(content: &str, keywords: &[String]) -> bool {
    if keywords.is_empty() {
        return true;
    }
    let lower = content.to_lowercase();
    keywords.iter().any(|kw| lower.contains(kw))
}

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let config = Config::from_env();

    let verifier = SignatureVerifier::new(
        SignatureConfig::from_bot_secret(&config.bot_secret)
            .expect("invalid bot secret")
            .with_replay_protection(ReplayProtectionMode::Monitor, Duration::from_secs(10 * 60)),
    )
    .expect("invalid bot secret");
    let hook = webhook_validation_hook(&config.bot_secret);
    let debug_webhook = env::var("QQ_WEBHOOK_DEBUG")
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false);
    let debug_limit = env::var("QQ_WEBHOOK_DEBUG_BODY_LIMIT")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(2048);
    let debug_log_path =
        env::var("QQ_WEBHOOK_DEBUG_LOG").unwrap_or_else(|_| "webhook_raw.log".to_string());
    let raw_logger: Option<RawLogger> = if debug_webhook {
        let debug_log_path = debug_log_path.clone();
        Some(Arc::new(move |headers: &http::HeaderMap, body: &[u8]| {
            let limit = debug_limit.min(body.len());
            let preview = String::from_utf8_lossy(&body[..limit]);
            tracing::info!(
                body_len = body.len(),
                body_preview = %preview,
                ?headers,
                "webhook raw request"
            );
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&debug_log_path)
            {
                let _ = writeln!(
                    file,
                    "----\nbody_len={}\nheaders={:?}\nbody_preview={}\n",
                    body.len(),
                    headers,
                    preview
                );
            }
        }))
    } else {
        None
    };

    let token_provider =
        HttpTokenProvider::from_env_or_official(&config.app_id, &config.client_secret);
    let token_manager = TokenManager::new(token_provider, Duration::from_secs(120));
    let client = OpenApiClient::new(token_manager, OpenApiConfig::from_env_or_official());
    let api = Arc::new(OpenApi::new(client, OpenApiPaths::official_defaults()));
    let c2c_auto_reply = env::var("QQ_C2C_AUTO_REPLY")
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false);
    let c2c_reply_prefix = env::var("QQ_C2C_REPLY_PREFIX").unwrap_or_else(|_| "echo: ".to_string());
    let c2c_reply_keywords = parse_keywords(&env::var("QQ_C2C_REPLY_KEYWORDS").unwrap_or_default());
    let c2c_deduper = Deduper::from_env();

    let router = EventRouter::new()
        .middleware(LoggingMiddleware)
        .route_fn("C2C_MESSAGE_CREATE", {
            let api = Arc::clone(&api);
            let c2c_reply_prefix = c2c_reply_prefix.clone();
            let c2c_reply_keywords = c2c_reply_keywords.clone();
            let c2c_deduper = c2c_deduper.clone();
            move |ctx| {
                let api = Arc::clone(&api);
                let c2c_reply_prefix = c2c_reply_prefix.clone();
                let c2c_reply_keywords = c2c_reply_keywords.clone();
                let c2c_deduper = c2c_deduper.clone();
                async move {
                    if !c2c_auto_reply {
                        return Ok(EventResponse::ok());
                    }
                    let event = ctx.parse_typed_event()?;
                    if let EventData::C2cMessage(msg) = event.data {
                        let content = msg.content.unwrap_or_default();
                        if content.trim().is_empty() {
                            return Ok(EventResponse::ok());
                        }
                        if !matches_keywords(&content, &c2c_reply_keywords) {
                            return Ok(EventResponse::ok());
                        }
                        if c2c_deduper.is_duplicate(&msg.id).await {
                            tracing::debug!(msg_id = %msg.id, "c2c duplicate ignored");
                            return Ok(EventResponse::ok());
                        }
                        let reply = format!("{}{}", c2c_reply_prefix, content);
                        let body = json!({
                            "msg_id": msg.id,
                            "msg_seq": msg.msg_seq.unwrap_or(1),
                            "msg_type": 0,
                            "content": reply,
                        });
                        match api.c2c_messages().send(&msg.author.user_openid, &body).await {
                            Ok((status, resp)) => {
                                tracing::info!(?status, ?resp, "c2c reply sent");
                            }
                            Err(err) => {
                                tracing::warn!(?err, "c2c reply failed");
                            }
                        }
                    }
                    Ok(EventResponse::ok())
                }
            }
        })
        .route_fn("INTERACTION_CREATE", {
            let api = Arc::clone(&api);
            move |ctx| {
                let api = Arc::clone(&api);
                async move {
                    let event = ctx.parse_typed_event()?;
                    if let EventData::InteractionCreate(interaction) = event.data {
                        let status = api.interactions().ack(&interaction.id, 0).await?;
                        tracing::info!(interaction_id = %interaction.id, ?status, "interaction acked");
                    }
                    Ok(EventResponse::ok())
                }
            }
        })
        .default_handler_fn(|_ctx| async move { Ok(EventResponse::ok()) });

    let webhook = WebhookApp::new(
        router,
        WebhookConfig {
            path: config.webhook_path.clone(),
            signature: Some(verifier),
            signature_verification: SignatureVerificationMode::Monitor,
            hook: Some(hook),
            raw_logger,
            event_name_extractor: event_name_field("t"),
            ..Default::default()
        },
    )
    .into_router();

    let app = Router::new().route("/healthz", get(healthz)).merge(webhook);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse().unwrap();
    tracing::info!(%addr, "listening");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
