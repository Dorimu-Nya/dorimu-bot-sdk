use super::{
    Method, OpenApiClient, OpenApiPaths, Result, TokenProvider, json, render_path, require_path,
};

/// 交互事件回包接口。
#[derive(Clone)]
pub struct InteractionsApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> InteractionsApi<P>
where
    P: TokenProvider,
{
    pub async fn ack(&self, interaction_id: &str, code: i64) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.interaction_put, "interaction_put")?;
        let path = render_path(&template, &[("interaction_id", interaction_id)])?;
        let body = json!({ "code": code });
        let resp = self
            .client
            .request_json(Method::PUT, &path, Some(&body))
            .await?;
        Ok(resp.status())
    }
}


