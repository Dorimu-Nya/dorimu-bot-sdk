use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{SendMessageRequest, SendMessageResponse};
use serde::Serialize;

/// C2C 消息发送接口。
#[derive(Clone)]
pub struct C2cMessagesApi<P> {
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> C2cMessagesApi<P>
where
    P: TokenProvider,
{
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

    pub async fn send_typed(
        &self,
        openid: &str,
        body: &SendMessageRequest,
    ) -> Result<(http::StatusCode, SendMessageResponse)> {
        self.send(openid, body).await
    }
}
