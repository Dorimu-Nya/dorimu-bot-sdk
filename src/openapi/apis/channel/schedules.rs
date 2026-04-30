use super::{
    append_query, render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result,
    TokenProvider,
};
use crate::openapi::models::{Schedule, SchedulesQuery, UpsertScheduleRequest};

/// 日程相关接口。
#[derive(Clone)]
pub struct SchedulesApi<P> {
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> SchedulesApi<P>
where
    P: TokenProvider,
{
    pub async fn list(&self, channel_id: &str) -> Result<(http::StatusCode, Vec<Schedule>)> {
        self.list_with(channel_id, None).await
    }

    pub async fn list_with(
        &self,
        channel_id: &str,
        since: Option<u64>,
    ) -> Result<(http::StatusCode, Vec<Schedule>)> {
        let template = require_path(&self.paths.schedule_list, "schedule_list")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        let path = append_query(path, &[("since", since.map(|v| v.to_string()))]);
        self.client.get_t(&path).await
    }

    pub async fn list_by_query(
        &self,
        channel_id: &str,
        query: &SchedulesQuery,
    ) -> Result<(http::StatusCode, Vec<Schedule>)> {
        self.list_with(channel_id, query.since).await
    }

    pub async fn get(
        &self,
        channel_id: &str,
        schedule_id: &str,
    ) -> Result<(http::StatusCode, Schedule)> {
        let template = require_path(&self.paths.schedule_get, "schedule_get")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("schedule_id", schedule_id)],
        )?;
        self.client.get_t(&path).await
    }

    pub async fn create(
        &self,
        channel_id: &str,
        body: &UpsertScheduleRequest,
    ) -> Result<(http::StatusCode, Schedule)> {
        let template = require_path(&self.paths.schedule_create, "schedule_create")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }

    pub async fn update(
        &self,
        channel_id: &str,
        schedule_id: &str,
        body: &UpsertScheduleRequest,
    ) -> Result<(http::StatusCode, Schedule)> {
        let template = require_path(&self.paths.schedule_update, "schedule_update")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("schedule_id", schedule_id)],
        )?;
        self.client
            .request_t_with(Method::PATCH, &path, Some(body))
            .await
    }

    pub async fn delete(&self, channel_id: &str, schedule_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.schedule_delete, "schedule_delete")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("schedule_id", schedule_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}
