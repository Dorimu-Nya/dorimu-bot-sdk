use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider, Value,
};

/// 群聊消息发送接口。
#[derive(Clone)]
pub struct GroupMessagesApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> GroupMessagesApi<P>
where
    P: TokenProvider,
{
    pub async fn send(
        &self,
        group_openid: &str,
        body: &Value,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.group_message_send, "group_message_send")?;
        let path = render_path(&template, &[("group_openid", group_openid)])?;
        self.client
            .request_value(Method::POST, &path, Some(body))
            .await
    }
}
