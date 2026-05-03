use super::{
    append_query, render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result,
    TokenProvider,
};
use crate::openapi::models::{DeleteMemberOptions, Member, MembersQuery};

/// 成员相关接口。
#[derive(Clone)]
pub struct MembersApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 成员接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> MembersApi<P>
where
    P: TokenProvider,
{
    /// 获取指定频道的成员列表。
    pub async fn list(&self, guild_id: &str) -> Result<(http::StatusCode, Vec<Member>)> {
        self.list_with(guild_id, None, None).await
    }

    /// 按分页参数获取指定频道的成员列表。
    pub async fn list_with(
        &self,
        guild_id: &str,
        after: Option<&str>,
        limit: Option<u64>,
    ) -> Result<(http::StatusCode, Vec<Member>)> {
        let template = require_path(&self.paths.member_list, "member_list")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        let path = append_query(
            path,
            &[
                ("after", after.map(|v| v.to_string())),
                ("limit", limit.map(|v| v.to_string())),
            ],
        );
        self.client.get_t(&path).await
    }

    /// 按查询参数获取指定频道的成员列表。
    pub async fn list_by_query(
        &self,
        guild_id: &str,
        query: &MembersQuery,
    ) -> Result<(http::StatusCode, Vec<Member>)> {
        self.list_with(
            guild_id,
            query.after.as_deref(),
            query.limit.map(|v| v as u64),
        )
        .await
    }

    /// 获取指定频道成员详情。
    pub async fn get(&self, guild_id: &str, user_id: &str) -> Result<(http::StatusCode, Member)> {
        let template = require_path(&self.paths.member_get, "member_get")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("user_id", user_id)])?;
        self.client.get_t(&path).await
    }

    /// 删除指定频道成员。
    pub async fn delete(&self, guild_id: &str, user_id: &str) -> Result<http::StatusCode> {
        self.delete_with(guild_id, user_id, None).await
    }

    /// 按选项删除指定频道成员。
    pub async fn delete_with(
        &self,
        guild_id: &str,
        user_id: &str,
        options: Option<&DeleteMemberOptions>,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.member_delete, "member_delete")?;
        let path = render_path(&template, &[("guild_id", guild_id), ("user_id", user_id)])?;
        let path = append_query(
            path,
            &[
                (
                    "add_blacklist",
                    options.and_then(|v| v.add_blacklist).map(|v| v.to_string()),
                ),
                (
                    "delete_history_msg_days",
                    options
                        .and_then(|v| v.delete_history_msg_days)
                        .map(|v| v.to_string()),
                ),
            ],
        );
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }
}
