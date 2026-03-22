use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider, Value,
};

/// 子频道权限相关接口。
#[derive(Clone)]
pub struct ChannelPermissionsApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> ChannelPermissionsApi<P>
where
    P: TokenProvider,
{
    pub async fn get_user(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(
            &self.paths.channel_permissions_get_user,
            "channel_permissions_get_user",
        )?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("user_id", user_id)],
        )?;
        self.client.get_value(&path).await
    }

    pub async fn set_user(
        &self,
        channel_id: &str,
        user_id: &str,
        body: &Value,
    ) -> Result<http::StatusCode> {
        let template = require_path(
            &self.paths.channel_permissions_set_user,
            "channel_permissions_set_user",
        )?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("user_id", user_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::PUT, &path, Some(body))
            .await?;
        Ok(resp.status())
    }

    pub async fn get_role(
        &self,
        channel_id: &str,
        role_id: &str,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(
            &self.paths.channel_permissions_get_role,
            "channel_permissions_get_role",
        )?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("role_id", role_id)],
        )?;
        self.client.get_value(&path).await
    }

    pub async fn set_role(
        &self,
        channel_id: &str,
        role_id: &str,
        body: &Value,
    ) -> Result<http::StatusCode> {
        let template = require_path(
            &self.paths.channel_permissions_set_role,
            "channel_permissions_set_role",
        )?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("role_id", role_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::PUT, &path, Some(body))
            .await?;
        Ok(resp.status())
    }
}
