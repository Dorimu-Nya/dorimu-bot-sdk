use super::{
    append_query, render_path, require_path, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{Guild, User, UserGuildsQuery};

/// 用户信息相关接口。
#[derive(Clone)]
pub struct UsersApi<P> {
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> UsersApi<P>
where
    P: TokenProvider,
{
    pub async fn me(&self) -> Result<(http::StatusCode, User)> {
        let template = require_path(&self.paths.user_me, "user_me")?;
        let path = render_path(&template, &[])?;
        self.client.get_t(&path).await
    }

    pub async fn guilds(
        &self,
        before: Option<&str>,
        after: Option<&str>,
        limit: Option<u64>,
    ) -> Result<(http::StatusCode, Vec<Guild>)> {
        let template = require_path(&self.paths.user_guilds, "user_guilds")?;
        let path = render_path(&template, &[])?;
        let path = append_query(
            path,
            &[
                ("before", before.map(|v| v.to_string())),
                ("after", after.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
            ],
        );
        self.client.get_t(&path).await
    }

    pub async fn guilds_with(
        &self,
        query: &UserGuildsQuery,
    ) -> Result<(http::StatusCode, Vec<Guild>)> {
        self.guilds(
            query.before.as_deref(),
            query.after.as_deref(),
            query.limit.map(|v| v as u64),
        )
        .await
    }
}
