use super::{OpenApiClient, OpenApiPaths, Result, TokenProvider, Value, append_query, render_path, require_path};

/// 用户信息相关接口。
#[derive(Clone)]
pub struct UsersApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> UsersApi<P>
where
    P: TokenProvider,
{
    pub async fn me(&self) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.user_me, "user_me")?;
        let path = render_path(&template, &[])?;
        self.client.get_value(&path).await
    }

    pub async fn guilds(
        &self,
        before: Option<&str>,
        after: Option<&str>,
        limit: Option<u64>,
    ) -> Result<(http::StatusCode, Value)> {
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
        self.client.get_value(&path).await
    }
}


