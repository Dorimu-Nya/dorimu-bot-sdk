use super::{
    append_query, render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result,
    TokenProvider,
};
use crate::openapi::models::{ReactionUsersQuery, ReactionUsersResponse};

/// 表情回应相关接口。
#[derive(Clone)]
pub struct ReactionsApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 表情回应接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> ReactionsApi<P>
where
    P: TokenProvider,
{
    /// 为指定消息添加表情回应。
    pub async fn add(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: &str,
        emoji_id: &str,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.reaction_add, "reaction_add")?;
        let path = render_path(
            &template,
            &[
                ("channel_id", channel_id),
                ("message_id", message_id),
                ("type", emoji_type),
                ("id", emoji_id),
            ],
        )?;
        let resp = self.client.request_json(Method::PUT, &path, None).await?;
        Ok(resp.status())
    }

    /// 删除指定消息上的表情回应。
    pub async fn delete(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: &str,
        emoji_id: &str,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.reaction_delete, "reaction_delete")?;
        let path = render_path(
            &template,
            &[
                ("channel_id", channel_id),
                ("message_id", message_id),
                ("type", emoji_type),
                ("id", emoji_id),
            ],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }

    /// 获取对指定消息添加某表情回应的用户列表。
    pub async fn users(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: &str,
        emoji_id: &str,
        cookie: Option<&str>,
        limit: Option<u64>,
    ) -> Result<(http::StatusCode, ReactionUsersResponse)> {
        let template = require_path(&self.paths.reaction_users, "reaction_users")?;
        let path = render_path(
            &template,
            &[
                ("channel_id", channel_id),
                ("message_id", message_id),
                ("type", emoji_type),
                ("id", emoji_id),
            ],
        )?;
        let path = append_query(
            path,
            &[
                ("cookie", cookie.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
            ],
        );
        self.client.get_t(&path).await
    }

    /// 按查询参数获取表情回应用户列表。
    pub async fn users_with(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: &str,
        emoji_id: &str,
        query: &ReactionUsersQuery,
    ) -> Result<(http::StatusCode, ReactionUsersResponse)> {
        self.users(
            channel_id,
            message_id,
            emoji_type,
            emoji_id,
            query.cookie.as_deref(),
            query.limit.map(|v| v as u64),
        )
        .await
    }
}
