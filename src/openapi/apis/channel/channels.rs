use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{Channel, OnlineNumsResponse, UpdateChannelRequest};

/// 子频道（Channel）相关接口。
#[derive(Clone)]
pub struct ChannelsApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 子频道相关接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> ChannelsApi<P>
where
    P: TokenProvider,
{
    /// 获取指定子频道详情。
    pub async fn get(&self, channel_id: &str) -> Result<(http::StatusCode, Channel)> {
        let template = require_path(&self.paths.channel_get, "channel_get")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_t(&path).await
    }

    /// 修改指定子频道信息。
    pub async fn update(
        &self,
        channel_id: &str,
        body: &UpdateChannelRequest,
    ) -> Result<(http::StatusCode, Channel)> {
        let template = require_path(&self.paths.channel_update, "channel_update")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client
            .request_t_with(Method::PATCH, &path, Some(body))
            .await
    }

    /// 删除指定子频道。
    pub async fn delete(&self, channel_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.channel_delete, "channel_delete")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }

    /// 获取指定子频道在线人数。
    pub async fn online_nums(
        &self,
        channel_id: &str,
    ) -> Result<(http::StatusCode, OnlineNumsResponse)> {
        let template = require_path(&self.paths.channel_online_nums, "channel_online_nums")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_t(&path).await
    }
}
