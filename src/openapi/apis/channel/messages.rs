use super::{
    append_query, render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result,
    TokenProvider,
};
use crate::openapi::models::{
    DeleteMessageOptions, Message, SendMessageRequest, SendMessageResponse, UpdateMessageRequest,
};
use serde::Serialize;

/// 子频道消息相关接口。
#[derive(Clone)]
pub struct ChannelMessagesApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 子频道消息接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> ChannelMessagesApi<P>
where
    P: TokenProvider,
{
    /// 向指定子频道发送消息。
    pub async fn send<B: Serialize + ?Sized>(
        &self,
        channel_id: &str,
        body: &B,
    ) -> Result<(http::StatusCode, SendMessageResponse)> {
        let template = require_path(&self.paths.channel_message_send, "channel_message_send")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }

    /// 使用强类型请求体向指定子频道发送消息。
    pub async fn send_typed(
        &self,
        channel_id: &str,
        body: &SendMessageRequest,
    ) -> Result<(http::StatusCode, SendMessageResponse)> {
        self.send(channel_id, body).await
    }

    /// 获取指定子频道消息详情。
    pub async fn get(
        &self,
        channel_id: &str,
        message_id: &str,
    ) -> Result<(http::StatusCode, Message)> {
        let template = require_path(&self.paths.channel_message_get, "channel_message_get")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("message_id", message_id)],
        )?;
        self.client.get_t(&path).await
    }

    /// 修改指定子频道 Markdown 消息。
    pub async fn update(
        &self,
        channel_id: &str,
        message_id: &str,
        body: &UpdateMessageRequest,
    ) -> Result<(http::StatusCode, Message)> {
        let template = require_path(&self.paths.channel_message_update, "channel_message_update")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("message_id", message_id)],
        )?;
        self.client
            .request_t_with(Method::PATCH, &path, Some(body))
            .await
    }

    /// 撤回指定子频道消息，使用默认提示行为。
    pub async fn delete(&self, channel_id: &str, message_id: &str) -> Result<http::StatusCode> {
        self.delete_with(channel_id, message_id, None).await
    }

    /// 撤回指定子频道消息，并可控制是否隐藏提示小灰条。
    pub async fn delete_with(
        &self,
        channel_id: &str,
        message_id: &str,
        options: Option<&DeleteMessageOptions>,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.channel_message_delete, "channel_message_delete")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("message_id", message_id)],
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
