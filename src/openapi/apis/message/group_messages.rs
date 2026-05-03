use super::{
    append_query, render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result,
    TokenProvider,
};
use crate::openapi::models::{
    GroupMembersQuery, GroupMembersResponse, SendMessageRequest, SendMessageResponse,
};
use serde::Serialize;

/// 群聊消息发送接口。
#[derive(Clone)]
pub struct GroupMessagesApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 群聊消息和群成员接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> GroupMessagesApi<P>
where
    P: TokenProvider,
{
    /// 向指定群聊发送消息。
    pub async fn send<B: Serialize + ?Sized>(
        &self,
        group_openid: &str,
        body: &B,
    ) -> Result<(http::StatusCode, SendMessageResponse)> {
        let template = require_path(&self.paths.group_message_send, "group_message_send")?;
        let path = render_path(&template, &[("group_openid", group_openid)])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }

    /// 使用强类型请求体向指定群聊发送消息。
    pub async fn send_typed(
        &self,
        group_openid: &str,
        body: &SendMessageRequest,
    ) -> Result<(http::StatusCode, SendMessageResponse)> {
        self.send(group_openid, body).await
    }

    /// 撤回指定群聊消息。
    pub async fn delete(&self, group_openid: &str, message_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.group_message_delete, "group_message_delete")?;
        let path = render_path(
            &template,
            &[("group_openid", group_openid), ("message_id", message_id)],
        )?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }

    /// 获取指定群聊成员列表。
    pub async fn members(
        &self,
        group_openid: &str,
        limit: Option<u32>,
        start_index: Option<u32>,
    ) -> Result<(http::StatusCode, GroupMembersResponse)> {
        let query = GroupMembersQuery { limit, start_index };
        self.members_with(group_openid, &query).await
    }

    /// 按查询参数获取指定群聊成员列表。
    pub async fn members_with(
        &self,
        group_openid: &str,
        query: &GroupMembersQuery,
    ) -> Result<(http::StatusCode, GroupMembersResponse)> {
        let template = require_path(&self.paths.group_members_list, "group_members_list")?;
        let path = render_path(&template, &[("group_openid", group_openid)])?;
        let path = append_query(
            path,
            &[
                ("limit", query.limit.map(|v| v.to_string())),
                ("start_index", query.start_index.map(|v| v.to_string())),
            ],
        );
        self.client
            .request_t_with(Method::POST, &path, Option::<&()>::None)
            .await
    }
}
