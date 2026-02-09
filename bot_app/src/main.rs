mod auth;
mod command;
mod config;
mod store;

use async_trait::async_trait;
use axum::{routing::get, Router};
use qqbot_sdk::{
    event_name_field, webhook_validation_hook, EventContext, EventData, EventResponse, EventRouter,
    HttpTokenProvider, Middleware, Next, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths,
    ReplayProtectionMode, SignatureConfig, SignatureVerificationMode, SignatureVerifier,
    TokenManager, WebhookApp, WebhookConfig,
};
use serde_json::json;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;

use crate::auth::AccessControl;
use crate::command::{help_text, parse_command, Command};
use crate::config::Config;
use crate::store::KvStore;

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

// 内存去重器，避免重复事件重复处理。
#[derive(Clone)]
struct Deduper {
    store: Arc<Mutex<HashMap<String, Instant>>>,
    ttl: Duration,
    cap: usize,
}

impl Deduper {
    fn from_env() -> Self {
        // TTL/容量可配置，容量为 0 时禁用去重。
        let ttl = std::env::var("QQ_C2C_DEDUP_TTL_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(60);
        let cap = std::env::var("QQ_C2C_DEDUP_CAP")
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
        // 先清理过期记录。
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

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // 加载运行配置与辅助组件。
    let config = Config::from_env();
    let access = AccessControl::from_env();
    let store = Arc::new(KvStore::load(&config.data_path).await);
    let deduper = Deduper::from_env();

    let verifier = SignatureVerifier::new(
        SignatureConfig::from_bot_secret(&config.bot_secret)
            .expect("invalid bot secret")
            .with_replay_protection(ReplayProtectionMode::Monitor, Duration::from_secs(10 * 60)),
    )
    .expect("invalid bot secret");
    let hook = webhook_validation_hook(&config.bot_secret);

    let token_provider =
        HttpTokenProvider::from_env_or_official(&config.app_id, &config.client_secret);
    let token_manager = TokenManager::new(token_provider, Duration::from_secs(120));
    let client = OpenApiClient::new(token_manager, OpenApiConfig::from_env_or_official());
    let api = Arc::new(OpenApi::new(client, OpenApiPaths::official_defaults()));

    // C2C 指令路由：只响应带前缀的消息。
    let router = EventRouter::new()
        .middleware(LoggingMiddleware)
        .route_fn("C2C_MESSAGE_CREATE", {
            let api = Arc::clone(&api);
            let access = access.clone();
            let store = Arc::clone(&store);
            let deduper = deduper.clone();
            let cmd_prefix = config.cmd_prefix.clone();
            move |ctx| {
                let api = Arc::clone(&api);
                let access = access.clone();
                let store = Arc::clone(&store);
                let deduper = deduper.clone();
                let cmd_prefix = cmd_prefix.clone();
                async move {
                    let event = ctx.parse_typed_event()?;
                    if let EventData::C2cMessage(msg) = event.data {
                        let content = msg.content.unwrap_or_default();
                        let content = content.trim();
                        if content.is_empty() {
                            return Ok(EventResponse::ok());
                        }
                        if !access.allowed(&msg.author.user_openid) {
                            return Ok(EventResponse::ok());
                        }
                        if !content.starts_with(&cmd_prefix) {
                            return Ok(EventResponse::ok());
                        }
                        if deduper.is_duplicate(&msg.id).await {
                            tracing::debug!(msg_id = %msg.id, "c2c duplicate ignored");
                            return Ok(EventResponse::ok());
                        }

                        let command =
                            parse_command(&cmd_prefix, content).unwrap_or(Command::Unknown);
                        let reply = handle_command(command, &cmd_prefix, &store).await;
                        if let Some(reply) = reply {
                            let body = json!({
                                "msg_id": msg.id,
                                "msg_seq": msg.msg_seq.unwrap_or(1),
                                "msg_type": 0,
                                "content": reply,
                            });
                            match api
                                .c2c_messages()
                                .send(&msg.author.user_openid, &body)
                                .await
                            {
                                Ok((status, resp)) => {
                                    tracing::info!(?status, ?resp, "c2c reply sent");
                                }
                                Err(err) => {
                                    tracing::warn!(?err, "c2c reply failed");
                                }
                            }
                        }
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

async fn handle_command(command: Command, prefix: &str, store: &KvStore) -> Option<String> {
    match command {
        Command::Ping => Some("pong".to_string()),
        Command::Help => Some(help_text(prefix)),
        Command::Get { key } => match store.get(&key).await {
            Ok(Some(value)) => Some(format!("{} = {}", key, value)),
            Ok(None) => Some(format!("未找到键: {}", key)),
            Err(_) => Some("读取失败，请稍后再试".to_string()),
        },
        Command::Set { key, value } => match store.set(&key, &value).await {
            Ok(_) => Some(format!("已保存: {}", key)),
            Err(_) => Some("写入失败，请稍后再试".to_string()),
        },
        Command::Unknown => Some(help_text(prefix)),
    }
}
