use crate::{Error, HttpClient, Result, RetryPolicy};
use async_trait::async_trait;
use http::header::HeaderName;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{Method, Response};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

const OFFICIAL_API_BASE_URL: &str = "https://api.sgroup.qq.com";
const OFFICIAL_TOKEN_URL: &str = "https://bots.qq.com/app/getAppAccessToken";
use tokio::sync::{Mutex, RwLock};

#[derive(Debug, Clone)]
pub struct AccessToken {
    pub token: String,
    pub expires_at: SystemTime,
}

#[async_trait]
pub trait TokenProvider: Send + Sync {
    async fn fetch_token(&self) -> Result<AccessToken>;
}

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
    pub fn new(provider: P, refresh_margin: Duration) -> Self {
        Self {
            provider,
            cache: Arc::new(RwLock::new(None)),
            refresh_lock: Arc::new(Mutex::new(())),
            refresh_margin,
        }
    }

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

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub header_name: HeaderName,
    pub prefix: Option<String>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            header_name: http::header::AUTHORIZATION,
            prefix: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OpenApiConfig {
    pub base_url: String,
    pub auth: AuthConfig,
    pub retry: RetryPolicy,
}

impl OpenApiConfig {
    pub fn official() -> Self {
        Self {
            base_url: OFFICIAL_API_BASE_URL.to_string(),
            auth: AuthConfig {
                header_name: http::header::AUTHORIZATION,
                prefix: Some("QQBot".to_string()),
            },
            retry: RetryPolicy::default(),
        }
    }

    pub fn from_env_or_official() -> Self {
        let mut config = Self::official();
        if let Ok(base_url) = std::env::var("QQ_API_BASE_URL") {
            if !base_url.trim().is_empty() {
                config.base_url = base_url;
            }
        }
        config
    }
}

#[derive(Clone)]
pub struct OpenApiClient<P> {
    http: HttpClient,
    token_manager: TokenManager<P>,
    config: OpenApiConfig,
}

impl<P> OpenApiClient<P>
where
    P: TokenProvider,
{
    pub fn new(token_manager: TokenManager<P>, config: OpenApiConfig) -> Self {
        let http = HttpClient::new(reqwest::Client::new(), config.retry.clone());
        Self {
            http,
            token_manager,
            config,
        }
    }

    pub async fn request_json(
        &self,
        method: Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Response> {
        let token = self.token_manager.get_token().await?;
        let url = join_url(&self.config.base_url, path);
        let mut builder = self.http.client().request(method, url);

        let auth_value = if let Some(prefix) = &self.config.auth.prefix {
            format!("{} {}", prefix, token)
        } else {
            token
        };

        builder = builder.header(self.config.auth.header_name.clone(), auth_value);
        if let Some(body) = body {
            builder = builder.json(body);
        }
        self.http.send_with_retry(builder).await
    }

    pub async fn request_value(
        &self,
        method: Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<(http::StatusCode, Value)> {
        let resp = self.request_json(method, path, body).await?;
        let status = resp.status();
        let json = resp.json().await.map_err(Error::Http)?;
        Ok((status, json))
    }

    pub async fn request_t<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<(http::StatusCode, T)> {
        let resp = self.request_json(method, path, body).await?;
        let status = resp.status();
        let parsed = resp.json().await.map_err(Error::Http)?;
        Ok((status, parsed))
    }

    pub async fn get_value(&self, path: &str) -> Result<(http::StatusCode, Value)> {
        self.request_value(Method::GET, path, None).await
    }

    pub async fn post_value(&self, path: &str, body: &Value) -> Result<(http::StatusCode, Value)> {
        self.request_value(Method::POST, path, Some(body)).await
    }
}

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

    pub fn official(app_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self::official_with_token_url(OFFICIAL_TOKEN_URL, app_id, client_secret)
    }

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

#[derive(Debug, Clone, Default)]
pub struct OpenApiPaths {
    pub guild_get: Option<String>,
    pub guild_channels: Option<String>,
    pub channel_get: Option<String>,
    pub channel_online_nums: Option<String>,
    pub member_list: Option<String>,
    pub member_get: Option<String>,
    pub member_delete: Option<String>,
    pub role_members_list: Option<String>,
    pub role_member_add: Option<String>,
    pub role_member_delete: Option<String>,
    pub interaction_put: Option<String>,
    pub reaction_add: Option<String>,
    pub reaction_delete: Option<String>,
    pub reaction_users: Option<String>,
    pub announces_create: Option<String>,
    pub announces_delete: Option<String>,
    pub api_permissions_list: Option<String>,
    pub api_permissions_create: Option<String>,
    pub channel_permissions_get_user: Option<String>,
    pub channel_permissions_set_user: Option<String>,
    pub channel_permissions_get_role: Option<String>,
    pub channel_permissions_set_role: Option<String>,
    pub pins_list: Option<String>,
    pub pins_add: Option<String>,
    pub pins_delete: Option<String>,
    pub role_list: Option<String>,
    pub role_create: Option<String>,
    pub role_update: Option<String>,
    pub role_delete: Option<String>,
    pub schedule_list: Option<String>,
    pub schedule_get: Option<String>,
    pub schedule_create: Option<String>,
    pub schedule_update: Option<String>,
    pub schedule_delete: Option<String>,
    pub forum_threads_list: Option<String>,
    pub forum_thread_get: Option<String>,
    pub forum_thread_create: Option<String>,
    pub forum_thread_delete: Option<String>,
    pub mute_all: Option<String>,
    pub mute_user: Option<String>,
    pub c2c_message_send: Option<String>,
    pub message_setting_get: Option<String>,
    pub user_me: Option<String>,
    pub user_guilds: Option<String>,
}

impl OpenApiPaths {
    pub fn official_defaults() -> Self {
        Self {
            guild_get: Some("/guilds/{guild_id}".to_string()),
            guild_channels: Some("/guilds/{guild_id}/channels".to_string()),
            channel_get: Some("/channels/{channel_id}".to_string()),
            channel_online_nums: Some("/channels/{channel_id}/online_nums".to_string()),
            member_list: Some("/guilds/{guild_id}/members".to_string()),
            member_get: Some("/guilds/{guild_id}/members/{user_id}".to_string()),
            member_delete: Some("/guilds/{guild_id}/members/{user_id}".to_string()),
            role_members_list: Some("/guilds/{guild_id}/roles/{role_id}/members".to_string()),
            role_member_add: Some(
                "/guilds/{guild_id}/members/{user_id}/roles/{role_id}".to_string(),
            ),
            role_member_delete: Some(
                "/guilds/{guild_id}/members/{user_id}/roles/{role_id}".to_string(),
            ),
            interaction_put: Some("/interactions/{interaction_id}".to_string()),
            reaction_add: Some(
                "/channels/{channel_id}/messages/{message_id}/reactions/{type}/{id}".to_string(),
            ),
            reaction_delete: Some(
                "/channels/{channel_id}/messages/{message_id}/reactions/{type}/{id}".to_string(),
            ),
            reaction_users: Some(
                "/channels/{channel_id}/messages/{message_id}/reactions/{type}/{id}".to_string(),
            ),
            announces_create: Some("/guilds/{guild_id}/announces".to_string()),
            announces_delete: Some("/guilds/{guild_id}/announces/{message_id}".to_string()),
            api_permissions_list: Some("/guilds/{guild_id}/api_permission".to_string()),
            api_permissions_create: Some("/guilds/{guild_id}/api_permission/demand".to_string()),
            channel_permissions_get_user: Some(
                "/channels/{channel_id}/members/{user_id}/permissions".to_string(),
            ),
            channel_permissions_set_user: Some(
                "/channels/{channel_id}/members/{user_id}/permissions".to_string(),
            ),
            channel_permissions_get_role: Some(
                "/channels/{channel_id}/roles/{role_id}/permissions".to_string(),
            ),
            channel_permissions_set_role: Some(
                "/channels/{channel_id}/roles/{role_id}/permissions".to_string(),
            ),
            pins_list: Some("/channels/{channel_id}/pins".to_string()),
            pins_add: Some("/channels/{channel_id}/pins/{message_id}".to_string()),
            pins_delete: Some("/channels/{channel_id}/pins/{message_id}".to_string()),
            role_list: Some("/guilds/{guild_id}/roles".to_string()),
            role_create: Some("/guilds/{guild_id}/roles".to_string()),
            role_update: Some("/guilds/{guild_id}/roles/{role_id}".to_string()),
            role_delete: Some("/guilds/{guild_id}/roles/{role_id}".to_string()),
            schedule_list: Some("/channels/{channel_id}/schedules".to_string()),
            schedule_get: Some("/channels/{channel_id}/schedules/{schedule_id}".to_string()),
            schedule_create: Some("/channels/{channel_id}/schedules".to_string()),
            schedule_update: Some("/channels/{channel_id}/schedules/{schedule_id}".to_string()),
            schedule_delete: Some("/channels/{channel_id}/schedules/{schedule_id}".to_string()),
            forum_threads_list: Some("/channels/{channel_id}/threads".to_string()),
            forum_thread_get: Some("/channels/{channel_id}/threads/{thread_id}".to_string()),
            forum_thread_create: Some("/channels/{channel_id}/threads".to_string()),
            forum_thread_delete: Some("/channels/{channel_id}/threads/{thread_id}".to_string()),
            mute_all: Some("/guilds/{guild_id}/mute".to_string()),
            mute_user: Some("/guilds/{guild_id}/members/{user_id}/mute".to_string()),
            c2c_message_send: Some("/v2/users/{openid}/messages".to_string()),
            message_setting_get: Some("/guilds/{guild_id}/message/setting".to_string()),
            user_me: Some("/users/@me".to_string()),
            user_guilds: Some("/users/@me/guilds".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct OpenApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> OpenApi<P>
where
    P: TokenProvider + Clone,
{
    pub fn new(client: OpenApiClient<P>, paths: OpenApiPaths) -> Self {
        Self { client, paths }
    }

    pub fn guilds(&self) -> GuildsApi<P> {
        GuildsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn channels(&self) -> ChannelsApi<P> {
        ChannelsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn members(&self) -> MembersApi<P> {
        MembersApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn interactions(&self) -> InteractionsApi<P> {
        InteractionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn reactions(&self) -> ReactionsApi<P> {
        ReactionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn announces(&self) -> AnnouncesApi<P> {
        AnnouncesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn api_permissions(&self) -> ApiPermissionsApi<P> {
        ApiPermissionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn channel_permissions(&self) -> ChannelPermissionsApi<P> {
        ChannelPermissionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn pins(&self) -> PinsApi<P> {
        PinsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn roles(&self) -> RolesApi<P> {
        RolesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn schedules(&self) -> SchedulesApi<P> {
        SchedulesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn forums(&self) -> ForumsApi<P> {
        ForumsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn mute(&self) -> MuteApi<P> {
        MuteApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn message_settings(&self) -> MessageSettingsApi<P> {
        MessageSettingsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn c2c_messages(&self) -> C2cMessagesApi<P> {
        C2cMessagesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn users(&self) -> UsersApi<P> {
        UsersApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }
}

#[derive(Clone)]
pub struct GuildsApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> GuildsApi<P>
where
    P: TokenProvider,
{
    pub async fn get(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.guild_get, "guild_get")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn channels(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.guild_channels, "guild_channels")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_value(&path).await
    }
}

#[derive(Clone)]
pub struct ChannelsApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> ChannelsApi<P>
where
    P: TokenProvider,
{
    pub async fn get(&self, channel_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.channel_get, "channel_get")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn online_nums(&self, channel_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.channel_online_nums, "channel_online_nums")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_value(&path).await
    }
}

#[derive(Clone)]
pub struct MembersApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> MembersApi<P>
where
    P: TokenProvider,
{
    pub async fn list(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        self.list_with(guild_id, None, None).await
    }

    pub async fn list_with(
        &self,
        guild_id: &str,
        after: Option<&str>,
        limit: Option<u64>,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.member_list, "member_list")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        let path = append_query(
            path,
            &[
                ("after", after.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
            ],
        );
        self.client.get_value(&path).await
    }

    pub async fn get(&self, guild_id: &str, user_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.member_get, "member_get")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("user_id", user_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn delete(&self, guild_id: &str, user_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.member_delete, "member_delete")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("user_id", user_id)])?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}

#[derive(Clone)]
pub struct InteractionsApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> InteractionsApi<P>
where
    P: TokenProvider,
{
    pub async fn ack(&self, interaction_id: &str, code: i64) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.interaction_put, "interaction_put")?;
        let path = render_path(&template, &[("interaction_id", interaction_id)])?;
        let body = json!({ "code": code });
        let resp = self
            .client
            .request_json(Method::PUT, &path, Some(&body))
            .await?;
        Ok(resp.status())
    }
}

#[derive(Clone)]
pub struct ReactionsApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> ReactionsApi<P>
where
    P: TokenProvider,
{
    pub async fn add(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: &str,
        emoji_id: &str,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.reaction_add, "reaction_add")?;
        let path = render_path(
            &template,
            &[
                ("channel_id", channel_id),
                ("message_id", message_id),
                ("type", emoji_type),
                ("id", emoji_id),
            ],
        )?;
        let resp = self.client.request_json(Method::PUT, &path, None).await?;
        Ok(resp.status())
    }

    pub async fn delete(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: &str,
        emoji_id: &str,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.reaction_delete, "reaction_delete")?;
        let path = render_path(
            &template,
            &[
                ("channel_id", channel_id),
                ("message_id", message_id),
                ("type", emoji_type),
                ("id", emoji_id),
            ],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }

    pub async fn users(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: &str,
        emoji_id: &str,
        cookie: Option<&str>,
        limit: Option<u64>,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.reaction_users, "reaction_users")?;
        let path = render_path(
            &template,
            &[
                ("channel_id", channel_id),
                ("message_id", message_id),
                ("type", emoji_type),
                ("id", emoji_id),
            ],
        )?;
        let path = append_query(
            path,
            &[
                ("cookie", cookie.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
            ],
        );
        self.client.get_value(&path).await
    }
}

#[derive(Clone)]
pub struct AnnouncesApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> AnnouncesApi<P>
where
    P: TokenProvider,
{
    pub async fn create(&self, guild_id: &str, body: &Value) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.announces_create, "announces_create")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client
            .request_value(Method::POST, &path, Some(body))
            .await
    }

    pub async fn delete(&self, guild_id: &str, message_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.announces_delete, "announces_delete")?;
        let path = render_path(
            &template,
            &[("guild_id", guild_id), ("message_id", message_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }

    pub async fn clear(&self, guild_id: &str) -> Result<http::StatusCode> {
        self.delete(guild_id, "all").await
    }
}

#[derive(Clone)]
pub struct ApiPermissionsApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> ApiPermissionsApi<P>
where
    P: TokenProvider,
{
    pub async fn list(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.api_permissions_list, "api_permissions_list")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn create(&self, guild_id: &str, body: &Value) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.api_permissions_create, "api_permissions_create")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client
            .request_value(Method::POST, &path, Some(body))
            .await
    }
}

#[derive(Clone)]
pub struct ChannelPermissionsApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> ChannelPermissionsApi<P>
where
    P: TokenProvider,
{
    pub async fn get_user(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(
            &self.paths.channel_permissions_get_user,
            "channel_permissions_get_user",
        )?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("user_id", user_id)],
        )?;
        self.client.get_value(&path).await
    }

    pub async fn set_user(
        &self,
        channel_id: &str,
        user_id: &str,
        body: &Value,
    ) -> Result<http::StatusCode> {
        let template = require_path(
            &self.paths.channel_permissions_set_user,
            "channel_permissions_set_user",
        )?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("user_id", user_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::PUT, &path, Some(body))
            .await?;
        Ok(resp.status())
    }

    pub async fn get_role(
        &self,
        channel_id: &str,
        role_id: &str,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(
            &self.paths.channel_permissions_get_role,
            "channel_permissions_get_role",
        )?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("role_id", role_id)],
        )?;
        self.client.get_value(&path).await
    }

    pub async fn set_role(
        &self,
        channel_id: &str,
        role_id: &str,
        body: &Value,
    ) -> Result<http::StatusCode> {
        let template = require_path(
            &self.paths.channel_permissions_set_role,
            "channel_permissions_set_role",
        )?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("role_id", role_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::PUT, &path, Some(body))
            .await?;
        Ok(resp.status())
    }
}

#[derive(Clone)]
pub struct PinsApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> PinsApi<P>
where
    P: TokenProvider,
{
    pub async fn list(&self, channel_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.pins_list, "pins_list")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn add(&self, channel_id: &str, message_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.pins_add, "pins_add")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("message_id", message_id)],
        )?;
        let resp = self.client.request_json(Method::PUT, &path, None).await?;
        Ok(resp.status())
    }

    pub async fn delete(&self, channel_id: &str, message_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.pins_delete, "pins_delete")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("message_id", message_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}

#[derive(Clone)]
pub struct RolesApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> RolesApi<P>
where
    P: TokenProvider,
{
    pub async fn list(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.role_list, "role_list")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn list_members(
        &self,
        guild_id: &str,
        role_id: &str,
        start_index: Option<&str>,
        limit: Option<u64>,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.role_members_list, "role_members_list")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("role_id", role_id)])?;
        let path = append_query(
            path,
            &[
                ("start_index", start_index.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
            ],
        );
        self.client.get_value(&path).await
    }

    pub async fn create(&self, guild_id: &str, body: &Value) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.role_create, "role_create")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client
            .request_value(Method::POST, &path, Some(body))
            .await
    }

    pub async fn update(
        &self,
        guild_id: &str,
        role_id: &str,
        body: &Value,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.role_update, "role_update")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("role_id", role_id)])?;
        self.client
            .request_value(Method::PATCH, &path, Some(body))
            .await
    }

    pub async fn delete(&self, guild_id: &str, role_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.role_delete, "role_delete")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("role_id", role_id)])?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }

    pub async fn add_member(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.role_member_add, "role_member_add")?;
        let path = render_path(
            &template,
            &[
                ("guild_id", guild_id),
                ("user_id", user_id),
                ("role_id", role_id),
            ],
        )?;
        let resp = self.client.request_json(Method::PUT, &path, None).await?;
        Ok(resp.status())
    }

    pub async fn remove_member(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.role_member_delete, "role_member_delete")?;
        let path = render_path(
            &template,
            &[
                ("guild_id", guild_id),
                ("user_id", user_id),
                ("role_id", role_id),
            ],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}

#[derive(Clone)]
pub struct SchedulesApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> SchedulesApi<P>
where
    P: TokenProvider,
{
    pub async fn list(&self, channel_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.schedule_list, "schedule_list")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn get(
        &self,
        channel_id: &str,
        schedule_id: &str,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.schedule_get, "schedule_get")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("schedule_id", schedule_id)],
        )?;
        self.client.get_value(&path).await
    }

    pub async fn create(
        &self,
        channel_id: &str,
        body: &Value,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.schedule_create, "schedule_create")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client
            .request_value(Method::POST, &path, Some(body))
            .await
    }

    pub async fn update(
        &self,
        channel_id: &str,
        schedule_id: &str,
        body: &Value,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.schedule_update, "schedule_update")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("schedule_id", schedule_id)],
        )?;
        self.client
            .request_value(Method::PATCH, &path, Some(body))
            .await
    }

    pub async fn delete(&self, channel_id: &str, schedule_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.schedule_delete, "schedule_delete")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("schedule_id", schedule_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}

#[derive(Clone)]
pub struct ForumsApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> ForumsApi<P>
where
    P: TokenProvider,
{
    pub async fn list_threads(&self, channel_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.forum_threads_list, "forum_threads_list")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn get_thread(
        &self,
        channel_id: &str,
        thread_id: &str,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.forum_thread_get, "forum_thread_get")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("thread_id", thread_id)],
        )?;
        self.client.get_value(&path).await
    }

    pub async fn create_thread(
        &self,
        channel_id: &str,
        body: &Value,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.forum_thread_create, "forum_thread_create")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client
            .request_value(Method::PUT, &path, Some(body))
            .await
    }

    pub async fn delete_thread(
        &self,
        channel_id: &str,
        thread_id: &str,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.forum_thread_delete, "forum_thread_delete")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("thread_id", thread_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}

#[derive(Clone)]
pub struct MuteApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> MuteApi<P>
where
    P: TokenProvider,
{
    pub async fn mute_all(&self, guild_id: &str, body: &Value) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.mute_all, "mute_all")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        let resp = self
            .client
            .request_json(Method::PATCH, &path, Some(body))
            .await?;
        Ok(resp.status())
    }

    pub async fn mute_user(
        &self,
        guild_id: &str,
        user_id: &str,
        body: &Value,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.mute_user, "mute_user")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("user_id", user_id)])?;
        let resp = self
            .client
            .request_json(Method::PATCH, &path, Some(body))
            .await?;
        Ok(resp.status())
    }
}

#[derive(Clone)]
pub struct MessageSettingsApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> MessageSettingsApi<P>
where
    P: TokenProvider,
{
    pub async fn get(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.message_setting_get, "message_setting_get")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_value(&path).await
    }
}

#[derive(Clone)]
pub struct C2cMessagesApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> C2cMessagesApi<P>
where
    P: TokenProvider,
{
    pub async fn send(&self, openid: &str, body: &Value) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.c2c_message_send, "c2c_message_send")?;
        let path = render_path(&template, &[("openid", openid)])?;
        self.client
            .request_value(Method::POST, &path, Some(body))
            .await
    }
}

#[derive(Clone)]
pub struct UsersApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> UsersApi<P>
where
    P: TokenProvider,
{
    pub async fn me(&self) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.user_me, "user_me")?;
        let path = render_path(&template, &[])?;
        self.client.get_value(&path).await
    }

    pub async fn guilds(
        &self,
        before: Option<&str>,
        after: Option<&str>,
        limit: Option<u64>,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.user_guilds, "user_guilds")?;
        let path = render_path(&template, &[])?;
        let path = append_query(
            path,
            &[
                ("before", before.map(|v| v.to_string())),
                ("after", after.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
            ],
        );
        self.client.get_value(&path).await
    }
}

fn join_url(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{}/{}", base, path)
}

fn require_path(path: &Option<String>, name: &str) -> Result<String> {
    path.clone()
        .ok_or_else(|| Error::Other(format!("missing openapi path template: {name}")))
}

fn render_path(template: &str, params: &[(&str, &str)]) -> Result<String> {
    let mut out = template.to_string();
    for (key, value) in params {
        let needle = format!("{{{}}}", key);
        if !out.contains(&needle) {
            return Err(Error::Other(format!(
                "path template missing placeholder: {needle}"
            )));
        }
        // 路径参数做百分号编码，避免 `/ ? # &` 等字符污染路由。
        let encoded = encode_path_segment(value);
        out = out.replace(&needle, &encoded);
    }
    Ok(out)
}

fn append_query(path: String, params: &[(&str, Option<String>)]) -> String {
    let mut parts = Vec::new();
    for (key, value) in params {
        if let Some(v) = value {
            // query key/value 分别编码，避免拼接注入。
            let encoded_key = encode_query_component(key);
            let encoded_value = encode_query_component(v);
            parts.push(format!("{encoded_key}={encoded_value}"));
        }
    }
    if parts.is_empty() {
        return path;
    }
    let separator = if path.contains('?') { "&" } else { "?" };
    format!("{path}{separator}{}", parts.join("&"))
}

fn encode_path_segment(value: &str) -> String {
    utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
}

fn encode_query_component(value: &str) -> String {
    utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
}

#[cfg(test)]
mod tests {
    use super::{append_query, render_path};

    #[test]
    fn render_path_percent_encodes_path_values() {
        let path = render_path("/v2/users/{openid}/messages", &[("openid", "user/a?b")]).unwrap();
        assert_eq!(path, "/v2/users/user%2Fa%3Fb/messages");
    }

    #[test]
    fn append_query_percent_encodes_query_values() {
        let path = append_query(
            "/users/@me/guilds".to_string(),
            &[
                ("before", Some("a/b&x=1".to_string())),
                ("limit", Some("100".to_string())),
            ],
        );
        assert_eq!(path, "/users/@me/guilds?before=a%2Fb%26x%3D1&limit=100");
    }
}
