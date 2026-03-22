use super::{Method, OpenApiClient, OpenApiPaths, Result, TokenProvider, Value, render_path, require_path};

/// 日程相关接口。
#[derive(Clone)]
pub struct SchedulesApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> SchedulesApi<P>
where
    P: TokenProvider,
{
    pub async fn list(&self, channel_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.schedule_list, "schedule_list")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn get(
        &self,
        channel_id: &str,
        schedule_id: &str,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.schedule_get, "schedule_get")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("schedule_id", schedule_id)],
        )?;
        self.client.get_value(&path).await
    }

    pub async fn create(
        &self,
        channel_id: &str,
        body: &Value,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.schedule_create, "schedule_create")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client
            .request_value(Method::POST, &path, Some(body))
            .await
    }

    pub async fn update(
        &self,
        channel_id: &str,
        schedule_id: &str,
        body: &Value,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.schedule_update, "schedule_update")?;
        let path = render_path(
            &template,
            &[("channel_id", channel_id), ("schedule_id", schedule_id)],
        )?;
        self.client
            .request_value(Method::PATCH, &path, Some(body))
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


