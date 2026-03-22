use super::{Method, OpenApiClient, OpenApiPaths, Result, TokenProvider, Value, render_path, require_path};

/// 禁言相关接口。
#[derive(Clone)]
pub struct MuteApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> MuteApi<P>
where
    P: TokenProvider,
{
    pub async fn mute_all(&self, guild_id: &str, body: &Value) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.mute_all, "mute_all")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        let resp = self
            .client
            .request_json(Method::PATCH, &path, Some(body))
            .await?;
        Ok(resp.status())
    }

    pub async fn mute_user(
        &self,
        guild_id: &str,
        user_id: &str,
        body: &Value,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.mute_user, "mute_user")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("user_id", user_id)])?;
        let resp = self
            .client
            .request_json(Method::PATCH, &path, Some(body))
            .await?;
        Ok(resp.status())
    }
}


