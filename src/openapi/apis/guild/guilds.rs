use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{Channel, CreateChannelRequest, Guild};

/// 频道（Guild）相关接口。
#[derive(Clone)]
pub struct GuildsApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 频道接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> GuildsApi<P>
where
    P: TokenProvider,
{
    /// 获取指定频道详情。
    pub async fn get(&self, guild_id: &str) -> Result<(http::StatusCode, Guild)> {
        let template = require_path(&self.paths.guild_get, "guild_get")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_t(&path).await
    }

    /// 获取指定频道下的子频道列表。
    pub async fn channels(&self, guild_id: &str) -> Result<(http::StatusCode, Vec<Channel>)> {
        let template = require_path(&self.paths.guild_channels, "guild_channels")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_t(&path).await
    }

    /// 在指定频道下创建子频道。
    pub async fn create_channel(
        &self,
        guild_id: &str,
        body: &CreateChannelRequest,
    ) -> Result<(http::StatusCode, Channel)> {
        let template = require_path(&self.paths.guild_channel_create, "guild_channel_create")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }
}
