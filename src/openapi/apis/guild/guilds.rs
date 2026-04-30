use super::{render_path, require_path, OpenApiClient, OpenApiPaths, Result, TokenProvider};
use crate::openapi::models::{Channel, Guild};

/// 频道（Guild）相关接口。
#[derive(Clone)]
pub struct GuildsApi<P> {
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> GuildsApi<P>
where
    P: TokenProvider,
{
    pub async fn get(&self, guild_id: &str) -> Result<(http::StatusCode, Guild)> {
        let template = require_path(&self.paths.guild_get, "guild_get")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_t(&path).await
    }

    pub async fn channels(&self, guild_id: &str) -> Result<(http::StatusCode, Vec<Channel>)> {
        let template = require_path(&self.paths.guild_channels, "guild_channels")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_t(&path).await
    }
}
