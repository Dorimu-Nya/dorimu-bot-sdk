use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider, Value,
};

/// C2C 消息发送接口。
#[derive(Clone)]
pub struct C2cMessagesApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
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
