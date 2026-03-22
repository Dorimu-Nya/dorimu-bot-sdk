use super::{OpenApiClient, OpenApiPaths, Result, TokenProvider, Value, render_path, require_path};

/// 子频道（Channel）相关接口。
#[derive(Clone)]
pub struct ChannelsApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> ChannelsApi<P>
where
    P: TokenProvider,
{
    pub async fn get(&self, channel_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.channel_get, "channel_get")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn online_nums(&self, channel_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.channel_online_nums, "channel_online_nums")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_value(&path).await
    }
}


