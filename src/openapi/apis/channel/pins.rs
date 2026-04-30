use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::PinsMessage;

/// 精华消息（Pins）相关接口。
#[derive(Clone)]
pub struct PinsApi<P> {
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> PinsApi<P>
where
    P: TokenProvider,
{
    pub async fn list(&self, channel_id: &str) -> Result<(http::StatusCode, PinsMessage)> {
        let template = require_path(&self.paths.pins_list, "pins_list")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_t(&path).await
    }

    pub async fn add(
        &self,
        channel_id: &str,
        message_id: &str,
    ) -> Result<(http::StatusCode, PinsMessage)> {
        let template = require_path(&self.paths.pins_add, "pins_add")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("message_id", message_id)],
        )?;
        self.client.request_t(Method::PUT, &path, None).await
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
