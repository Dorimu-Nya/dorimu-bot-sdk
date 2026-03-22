use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider, Value,
};

/// API 权限申请相关接口。
#[derive(Clone)]
pub struct ApiPermissionsApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> ApiPermissionsApi<P>
where
    P: TokenProvider,
{
    pub async fn list(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.api_permissions_list, "api_permissions_list")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn create(&self, guild_id: &str, body: &Value) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.api_permissions_create, "api_permissions_create")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client
            .request_value(Method::POST, &path, Some(body))
            .await
    }
}
