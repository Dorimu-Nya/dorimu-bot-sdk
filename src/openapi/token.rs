use crate::{Error, HttpClient, Result, RetryPolicy};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{Mutex, RwLock};

use super::config::OFFICIAL_TOKEN_URL;

/// 访问令牌及其过期时间。
#[derive(Debug, Clone)]
pub struct AccessToken {
    /// 令牌字符串。
    pub token: String,
    /// 令牌失效时间点。
    pub expires_at: SystemTime,
}

/// 令牌提供者抽象。
#[async_trait]
pub trait TokenProvider: Send + Sync {
    /// 拉取一个可用的访问令牌。
    async fn fetch_token(&self) -> Result<AccessToken>;
}

/// 带缓存与单飞刷新的令牌管理器。
#[derive(Clone)]
pub struct TokenManager<P> {
    provider: P,
    cache: Arc<RwLock<Option<AccessToken>>>,
    // 仅用于串行化刷新流程，避免 token 过期瞬间触发并发刷新风暴。
    refresh_lock: Arc<Mutex<()>>,
    refresh_margin: Duration,
}

impl<P> TokenManager<P>
where
    P: TokenProvider,
{
    /// 创建令牌管理器。
    ///
    /// `refresh_margin` 用于在过期前提前刷新，降低临界过期导致的请求失败概率。
    pub fn new(provider: P, refresh_margin: Duration) -> Self {
        Self {
            provider,
            cache: Arc::new(RwLock::new(None)),
            refresh_lock: Arc::new(Mutex::new(())),
            refresh_margin,
        }
    }

    /// 获取当前可用 token；必要时自动刷新并更新缓存。
    pub async fn get_token(&self) -> Result<String> {
        let now = SystemTime::now();
        {
            let guard = self.cache.read().await;
            if let Some(token) = guard.as_ref() {
                if token.expires_at > now + self.refresh_margin {
                    return Ok(token.token.clone());
                }
            }
        }

        // 首轮读取未命中后，进入单飞锁。
        let _refresh_guard = self.refresh_lock.lock().await;
        // 等待锁期间可能耗时较长，刷新时间戳避免误判。
        let now = SystemTime::now();
        {
            // 双重检查：等待锁期间可能已有其他协程刷新成功。
            let guard = self.cache.read().await;
            if let Some(token) = guard.as_ref() {
                if token.expires_at > now + self.refresh_margin {
                    return Ok(token.token.clone());
                }
            }
        }

        let new_token = self.provider.fetch_token().await?;
        let mut guard = self.cache.write().await;
        guard.replace(new_token.clone());
        Ok(new_token.token)
    }
}

/// 基于 HTTP 接口拉取 token 的默认实现。
#[derive(Clone)]
pub struct HttpTokenProvider {
    http: HttpClient,
    token_url: String,
    app_id: String,
    client_secret: String,
    body_builder: TokenBodyBuilder,
    token_pointer: String,
    expires_in_pointer: Option<String>,
    default_ttl: Duration,
}

type TokenBodyBuilder = Arc<dyn Fn(&str, &str) -> Value + Send + Sync>;

impl HttpTokenProvider {
    /// 创建一个可自定义请求体和响应指针的 token provider。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        token_url: impl Into<String>,
        app_id: impl Into<String>,
        client_secret: impl Into<String>,
        body_builder: TokenBodyBuilder,
        token_pointer: impl Into<String>,
        expires_in_pointer: Option<String>,
        default_ttl: Duration,
        retry: RetryPolicy,
    ) -> Self {
        let http = HttpClient::new(reqwest::Client::new(), retry);
        Self {
            http,
            token_url: token_url.into(),
            app_id: app_id.into(),
            client_secret: client_secret.into(),
            body_builder,
            token_pointer: token_pointer.into(),
            expires_in_pointer,
            default_ttl,
        }
    }

    /// 创建 QQ 官方 token provider。
    pub fn official(app_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self::official_with_token_url(OFFICIAL_TOKEN_URL, app_id, client_secret)
    }

    /// 使用指定 token URL 创建 QQ 官方格式 provider。
    pub fn official_with_token_url(
        token_url: impl Into<String>,
        app_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self::new(
            token_url,
            app_id,
            client_secret,
            Arc::new(
                |app_id, client_secret| json!({ "appId": app_id, "clientSecret": client_secret }),
            ),
            "/access_token",
            Some("/expires_in".to_string()),
            Duration::from_secs(7200),
            RetryPolicy::default(),
        )
    }

    /// 优先读取 `QQ_TOKEN_URL`，否则使用官方 token URL。
    pub fn from_env_or_official(
        app_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        let token_url = std::env::var("QQ_TOKEN_URL")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| OFFICIAL_TOKEN_URL.to_string());
        Self::official_with_token_url(token_url, app_id, client_secret)
    }
}

#[async_trait]
impl TokenProvider for HttpTokenProvider {
    async fn fetch_token(&self) -> Result<AccessToken> {
        let body = (self.body_builder)(&self.app_id, &self.client_secret);
        let builder = self.http.client().post(&self.token_url).json(&body);
        let resp = self.http.send_with_retry(builder).await?;
        let status = resp.status();
        if !status.is_success() {
            // 非 2xx 明确返回 Token 错误，避免误把错误体当成 token 响应解析。
            let body = resp.text().await.unwrap_or_default();
            let mut preview = body;
            if preview.len() > 512 {
                preview.truncate(512);
            }
            return Err(Error::Token(format!(
                "token endpoint returned status {}: {}",
                status.as_u16(),
                preview
            )));
        }
        let json: Value = resp.json().await.map_err(Error::Http)?;

        let token = json
            .pointer(&self.token_pointer)
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Token("access token not found in response".to_string()))?;

        let ttl = match &self.expires_in_pointer {
            Some(ptr) => json
                .pointer(ptr)
                .and_then(|v| v.as_u64())
                .map(Duration::from_secs)
                .unwrap_or(self.default_ttl),
            None => self.default_ttl,
        };

        Ok(AccessToken {
            token: token.to_string(),
            expires_at: SystemTime::now() + ttl,
        })
    }
}
