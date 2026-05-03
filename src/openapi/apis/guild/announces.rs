use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{Announces, CreateAnnouncesRequest};

/// 公告相关接口。
#[derive(Clone)]
pub struct AnnouncesApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 公告接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> AnnouncesApi<P>
where
    P: TokenProvider,
{
    /// 创建频道公告。
    pub async fn create(
        &self,
        guild_id: &str,
        body: &CreateAnnouncesRequest,
    ) -> Result<(http::StatusCode, Announces)> {
        let template = require_path(&self.paths.announces_create, "announces_create")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }

    /// 删除指定频道公告。
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

    /// 清空指定频道公告。
    pub async fn clear(&self, guild_id: &str) -> Result<http::StatusCode> {
        self.delete(guild_id, "all").await
    }

    /// 创建子频道公告。
    pub async fn create_channel(
        &self,
        channel_id: &str,
        body: &CreateAnnouncesRequest,
    ) -> Result<(http::StatusCode, Announces)> {
        let template = require_path(
            &self.paths.channel_announces_create,
            "channel_announces_create",
        )?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }

    /// 删除指定子频道公告。
    pub async fn delete_channel(
        &self,
        channel_id: &str,
        message_id: &str,
    ) -> Result<http::StatusCode> {
        let template = require_path(
            &self.paths.channel_announces_delete,
            "channel_announces_delete",
        )?;
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

    /// 清空指定子频道公告。
    pub async fn clear_channel(&self, channel_id: &str) -> Result<http::StatusCode> {
        self.delete_channel(channel_id, "all").await
    }
}
