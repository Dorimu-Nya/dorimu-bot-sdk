use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{
    GuildMuteMultiMemberRequest, GuildMuteMultiMemberResponse, GuildMuteRequest,
};

/// 禁言相关接口。
#[derive(Clone)]
pub struct MuteApi<P> {
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> MuteApi<P>
where
    P: TokenProvider,
{
    pub async fn mute_all(
        &self,
        guild_id: &str,
        body: &GuildMuteRequest,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.mute_all, "mute_all")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        let resp = self
            .client
            .request_json_with(Method::PATCH, &path, Some(body))
            .await?;
        Ok(resp.status())
    }

    pub async fn mute_user(
        &self,
        guild_id: &str,
        user_id: &str,
        body: &GuildMuteRequest,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.mute_user, "mute_user")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("user_id", user_id)])?;
        let resp = self
            .client
            .request_json_with(Method::PATCH, &path, Some(body))
            .await?;
        Ok(resp.status())
    }

    pub async fn mute_multi_members(
        &self,
        guild_id: &str,
        body: &GuildMuteMultiMemberRequest,
    ) -> Result<(http::StatusCode, GuildMuteMultiMemberResponse)> {
        let template = require_path(&self.paths.mute_all, "mute_all")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client
            .request_t_with(Method::PATCH, &path, Some(body))
            .await
    }
}
