use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{UploadMediaRequest, UploadMediaResponse};

/// 富媒体资源上传接口。
#[derive(Clone)]
pub struct MediaApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 富媒体上传接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> MediaApi<P>
where
    P: TokenProvider,
{
    /// 为 C2C 单聊上传富媒体资源。
    pub async fn upload_c2c(
        &self,
        openid: &str,
        body: &UploadMediaRequest,
    ) -> Result<(http::StatusCode, UploadMediaResponse)> {
        let template = require_path(&self.paths.c2c_file_upload, "c2c_file_upload")?;
        let path = render_path(&template, &[("openid", openid)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }

    /// 为群聊上传富媒体资源。
    pub async fn upload_group(
        &self,
        group_openid: &str,
        body: &UploadMediaRequest,
    ) -> Result<(http::StatusCode, UploadMediaResponse)> {
        let template = require_path(&self.paths.group_file_upload, "group_file_upload")?;
        let path = render_path(&template, &[("group_openid", group_openid)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }
}
