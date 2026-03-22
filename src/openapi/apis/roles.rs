use super::{
    append_query, render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result,
    TokenProvider, Value,
};

/// 身份组（Role）相关接口。
#[derive(Clone)]
pub struct RolesApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> RolesApi<P>
where
    P: TokenProvider,
{
    pub async fn list(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.role_list, "role_list")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn list_members(
        &self,
        guild_id: &str,
        role_id: &str,
        start_index: Option<&str>,
        limit: Option<u64>,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.role_members_list, "role_members_list")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("role_id", role_id)])?;
        let path = append_query(
            path,
            &[
                ("start_index", start_index.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
            ],
        );
        self.client.get_value(&path).await
    }

    pub async fn create(&self, guild_id: &str, body: &Value) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.role_create, "role_create")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client
            .request_value(Method::POST, &path, Some(body))
            .await
    }

    pub async fn update(
        &self,
        guild_id: &str,
        role_id: &str,
        body: &Value,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.role_update, "role_update")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("role_id", role_id)])?;
        self.client
            .request_value(Method::PATCH, &path, Some(body))
            .await
    }

    pub async fn delete(&self, guild_id: &str, role_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.role_delete, "role_delete")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("role_id", role_id)])?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }

    pub async fn add_member(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.role_member_add, "role_member_add")?;
        let path = render_path(
            &template,
            &[
                ("guild_id", guild_id),
                ("user_id", user_id),
                ("role_id", role_id),
            ],
        )?;
        let resp = self.client.request_json(Method::PUT, &path, None).await?;
        Ok(resp.status())
    }

    pub async fn remove_member(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.role_member_delete, "role_member_delete")?;
        let path = render_path(
            &template,
            &[
                ("guild_id", guild_id),
                ("user_id", user_id),
                ("role_id", role_id),
            ],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}
