use super::{
    append_query, render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result,
    TokenProvider,
};
use crate::openapi::models::{CreateDmsRequest, Dms, Guild, User, UserGuildsQuery};

/// 用户信息相关接口。
#[derive(Clone)]
pub struct UsersApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 用户接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> UsersApi<P>
where
    P: TokenProvider,
{
    /// 获取当前机器人用户信息。
    pub async fn me(&self) -> Result<(http::StatusCode, User)> {
        let template = require_path(&self.paths.user_me, "user_me")?;
        let path = render_path(&template, &[])?;
        self.client.get_t(&path).await
    }

    /// 获取当前机器人加入的频道列表。
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

    /// 按查询参数获取当前机器人加入的频道列表。
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

    /// 创建与指定用户的私信会话。
    pub async fn create_dms(&self, body: &CreateDmsRequest) -> Result<(http::StatusCode, Dms)> {
        let template = require_path(&self.paths.dms_create, "dms_create")?;
        let path = render_path(&template, &[])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }
}
