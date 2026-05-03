use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{
    ApiPermissionDemand, ApiPermissionsResponse, CreateApiPermissionDemandRequest,
};

/// API 权限申请相关接口。
#[derive(Clone)]
pub struct ApiPermissionsApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// API 权限接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> ApiPermissionsApi<P>
where
    P: TokenProvider,
{
    /// 获取指定频道可用的 API 权限列表。
    pub async fn list(&self, guild_id: &str) -> Result<(http::StatusCode, ApiPermissionsResponse)> {
        let template = require_path(&self.paths.api_permissions_list, "api_permissions_list")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_t(&path).await
    }

    /// 创建 API 权限需求申请。
    pub async fn create(
        &self,
        guild_id: &str,
        body: &CreateApiPermissionDemandRequest,
    ) -> Result<(http::StatusCode, ApiPermissionDemand)> {
        let template = require_path(&self.paths.api_permissions_create, "api_permissions_create")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }
}
