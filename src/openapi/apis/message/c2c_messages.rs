use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{SendMessageRequest, SendMessageResponse};
use serde::Serialize;

/// C2C 消息发送接口。
#[derive(Clone)]
pub struct C2cMessagesApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// C2C 消息接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> C2cMessagesApi<P>
where
    P: TokenProvider,
{
    /// 向指定 C2C 用户发送消息。
    pub async fn send<B: Serialize + ?Sized>(
        &self,
        openid: &str,
        body: &B,
    ) -> Result<(http::StatusCode, SendMessageResponse)> {
        let template = require_path(&self.paths.c2c_message_send, "c2c_message_send")?;
        let path = render_path(&template, &[("openid", openid)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }

    /// 使用强类型请求体向指定 C2C 用户发送消息。
    pub async fn send_typed(
        &self,
        openid: &str,
        body: &SendMessageRequest,
    ) -> Result<(http::StatusCode, SendMessageResponse)> {
        self.send(openid, body).await
    }

    /// 撤回指定 C2C 单聊消息。
    pub async fn delete(&self, openid: &str, message_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.c2c_message_delete, "c2c_message_delete")?;
        let path = render_path(&template, &[("openid", openid), ("message_id", message_id)])?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}
