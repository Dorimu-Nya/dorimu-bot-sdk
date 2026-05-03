use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{InteractionAckCode, InteractionAckRequest};
use crate::Error;

/// 交互事件回包接口。
#[derive(Clone)]
pub struct InteractionsApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 互动回调接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> InteractionsApi<P>
where
    P: TokenProvider,
{
    /// 使用整数回包码确认指定互动事件。
    pub async fn ack(&self, interaction_id: &str, code: i64) -> Result<http::StatusCode> {
        let code = match code {
            0 => InteractionAckCode::Success,
            1 => InteractionAckCode::Failed,
            2 => InteractionAckCode::TooFrequent,
            3 => InteractionAckCode::Duplicated,
            4 => InteractionAckCode::NoPermission,
            5 => InteractionAckCode::AdminOnly,
            _ => {
                return Err(Error::Other(format!(
                    "invalid interaction ack code: {code}, expected 0..=5"
                )));
            }
        };
        let body = InteractionAckRequest { code };
        self.ack_with(interaction_id, &body).await
    }

    /// 使用强类型请求体确认指定互动事件。
    pub async fn ack_with(
        &self,
        interaction_id: &str,
        body: &InteractionAckRequest,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.interaction_put, "interaction_put")?;
        let path = render_path(&template, &[("interaction_id", interaction_id)])?;
        let resp = self
            .client
            .request_json_with(Method::PUT, &path, Some(body))
            .await?;
        Ok(resp.status())
    }
}
