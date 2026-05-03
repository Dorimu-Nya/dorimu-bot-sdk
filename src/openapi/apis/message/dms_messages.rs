use super::{
    append_query, render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result,
    TokenProvider,
};
use crate::openapi::models::{DeleteMessageOptions, SendMessageRequest, SendMessageResponse};
use serde::Serialize;

/// 私信消息相关接口。
#[derive(Clone)]
pub struct DmsMessagesApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 私信消息接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> DmsMessagesApi<P>
where
    P: TokenProvider,
{
    /// 向指定私信会话发送消息。
    pub async fn send<B: Serialize + ?Sized>(
        &self,
        guild_id: &str,
        body: &B,
    ) -> Result<(http::StatusCode, SendMessageResponse)> {
        let template = require_path(&self.paths.dms_message_send, "dms_message_send")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }

    /// 使用强类型请求体向指定私信会话发送消息。
    pub async fn send_typed(
        &self,
        guild_id: &str,
        body: &SendMessageRequest,
    ) -> Result<(http::StatusCode, SendMessageResponse)> {
        self.send(guild_id, body).await
    }

    /// 撤回指定私信消息，使用默认提示行为。
    pub async fn delete(&self, guild_id: &str, message_id: &str) -> Result<http::StatusCode> {
        self.delete_with(guild_id, message_id, None).await
    }

    /// 撤回指定私信消息，并可控制是否隐藏提示小灰条。
    pub async fn delete_with(
        &self,
        guild_id: &str,
        message_id: &str,
        options: Option<&DeleteMessageOptions>,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.dms_message_delete, "dms_message_delete")?;
        let path = render_path(
            &template,
            &[("guild_id", guild_id), ("message_id", message_id)],
        )?;
        let path = append_query(
            path,
            &[(
                "hidetip",
                options.and_then(|o| o.hidetip).map(|v| v.to_string()),
            )],
        );
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}
