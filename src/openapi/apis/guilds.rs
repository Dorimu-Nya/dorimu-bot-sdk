use super::{OpenApiClient, OpenApiPaths, Result, TokenProvider, Value, render_path, require_path};

/// 频道（Guild）相关接口。
#[derive(Clone)]
pub struct GuildsApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> GuildsApi<P>
where
    P: TokenProvider,
{
    pub async fn get(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.guild_get, "guild_get")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn channels(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.guild_channels, "guild_channels")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_value(&path).await
    }
}


