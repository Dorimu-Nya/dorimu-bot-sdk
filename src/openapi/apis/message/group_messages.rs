use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{SendMessageRequest, SendMessageResponse};
use serde::Serialize;

/// 群聊消息发送接口。
#[derive(Clone)]
pub struct GroupMessagesApi<P> {
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> GroupMessagesApi<P>
where
    P: TokenProvider,
{
    pub async fn send<B: Serialize + ?Sized>(
        &self,
        group_openid: &str,
        body: &B,
    ) -> Result<(http::StatusCode, SendMessageResponse)> {
        let template = require_path(&self.paths.group_message_send, "group_message_send")?;
        let path = render_path(&template, &[("group_openid", group_openid)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }

    pub async fn send_typed(
        &self,
        group_openid: &str,
        body: &SendMessageRequest,
    ) -> Result<(http::StatusCode, SendMessageResponse)> {
        self.send(group_openid, body).await
    }
}
