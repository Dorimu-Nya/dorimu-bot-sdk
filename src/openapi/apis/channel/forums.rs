use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{
    CreateThreadRequest, CreateThreadResponse, ThreadDetailResponse, ThreadsListResponse,
};

/// 论坛帖子相关接口。
#[derive(Clone)]
pub struct ForumsApi<P> {
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> ForumsApi<P>
where
    P: TokenProvider,
{
    pub async fn list_threads(
        &self,
        channel_id: &str,
    ) -> Result<(http::StatusCode, ThreadsListResponse)> {
        let template = require_path(&self.paths.forum_threads_list, "forum_threads_list")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_t(&path).await
    }

    pub async fn get_thread(
        &self,
        channel_id: &str,
        thread_id: &str,
    ) -> Result<(http::StatusCode, ThreadDetailResponse)> {
        let template = require_path(&self.paths.forum_thread_get, "forum_thread_get")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("thread_id", thread_id)],
        )?;
        self.client.get_t(&path).await
    }

    pub async fn create_thread(
        &self,
        channel_id: &str,
        body: &CreateThreadRequest,
    ) -> Result<(http::StatusCode, CreateThreadResponse)> {
        let template = require_path(&self.paths.forum_thread_create, "forum_thread_create")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client
            .request_t_with(Method::PUT, &path, Some(body))
            .await
    }

    pub async fn delete_thread(
        &self,
        channel_id: &str,
        thread_id: &str,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.forum_thread_delete, "forum_thread_delete")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("thread_id", thread_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}
