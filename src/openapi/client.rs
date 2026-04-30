use crate::{Error, HttpClient, Result};
use reqwest::{Method, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use super::{
    config::OpenApiConfig,
    token::{TokenManager, TokenProvider},
    utils::join_url,
};

/// OpenAPI HTTP 客户端，统一处理鉴权与重试。
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
    /// 创建一个 OpenAPI 客户端。
    pub fn new(token_manager: TokenManager<P>, config: OpenApiConfig) -> Self {
        let http = HttpClient::new(reqwest::Client::new(), config.retry.clone());
        Self {
            http,
            token_manager,
            config,
        }
    }

    /// 发送请求并返回原始响应。
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

    /// 发送请求（泛型请求体）并返回原始响应。
    pub async fn request_json_with<B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
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

    /// 发送请求并将响应解析为 `serde_json::Value`。
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

    /// 发送请求并按目标类型反序列化。
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

    /// 发送请求（泛型请求体）并按目标类型反序列化。
    pub async fn request_t_with<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<(http::StatusCode, T)> {
        let resp = self.request_json_with(method, path, body).await?;
        let status = resp.status();
        let parsed = resp.json().await.map_err(Error::Http)?;
        Ok((status, parsed))
    }

    /// 发送 GET 请求并按目标类型反序列化。
    pub async fn get_t<T: DeserializeOwned>(&self, path: &str) -> Result<(http::StatusCode, T)> {
        self.request_t::<T>(Method::GET, path, None).await
    }

    /// 发送 GET 请求并返回 JSON。
    pub async fn get_value(&self, path: &str) -> Result<(http::StatusCode, Value)> {
        self.request_value(Method::GET, path, None).await
    }

    /// 发送 POST 请求并返回 JSON。
    pub async fn post_value(&self, path: &str, body: &Value) -> Result<(http::StatusCode, Value)> {
        self.request_value(Method::POST, path, Some(body)).await
    }
}
