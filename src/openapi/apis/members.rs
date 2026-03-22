use super::{
    append_query, render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result,
    TokenProvider, Value,
};

/// 成员相关接口。
#[derive(Clone)]
pub struct MembersApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> MembersApi<P>
where
    P: TokenProvider,
{
    pub async fn list(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        self.list_with(guild_id, None, None).await
    }

    pub async fn list_with(
        &self,
        guild_id: &str,
        after: Option<&str>,
        limit: Option<u64>,
    ) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.member_list, "member_list")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        let path = append_query(
            path,
            &[
                ("after", after.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
            ],
        );
        self.client.get_value(&path).await
    }

    pub async fn get(&self, guild_id: &str, user_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.member_get, "member_get")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("user_id", user_id)])?;
        self.client.get_value(&path).await
    }

    pub async fn delete(&self, guild_id: &str, user_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.member_delete, "member_delete")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("user_id", user_id)])?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}
