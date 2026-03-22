use super::{Method, OpenApiClient, OpenApiPaths, Result, TokenProvider, Value, render_path, require_path};

/// 精华消息（Pins）相关接口。
#[derive(Clone)]
pub struct PinsApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
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


