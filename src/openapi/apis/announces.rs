use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider, Value,
};

/// 公告相关接口。
#[derive(Clone)]
pub struct AnnouncesApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> AnnouncesApi<P>
where
    P: TokenProvider,
{
    pub async fn create(&self, guild_id: &str, body: &Value) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.announces_create, "announces_create")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client
            .request_value(Method::POST, &path, Some(body))
            .await
    }

    pub async fn delete(&self, guild_id: &str, message_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.announces_delete, "announces_delete")?;
        let path = render_path(
            &template,
            &[("guild_id", guild_id), ("message_id", message_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }

    pub async fn clear(&self, guild_id: &str) -> Result<http::StatusCode> {
        self.delete(guild_id, "all").await
    }
}
