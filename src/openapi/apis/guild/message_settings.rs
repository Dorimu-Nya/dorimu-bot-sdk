use super::{render_path, require_path, OpenApiClient, OpenApiPaths, Result, TokenProvider};
use crate::openapi::models::MessageSetting;

/// 发言设置查询接口。
#[derive(Clone)]
pub struct MessageSettingsApi<P> {
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> MessageSettingsApi<P>
where
    P: TokenProvider,
{
    pub async fn get(&self, guild_id: &str) -> Result<(http::StatusCode, MessageSetting)> {
        let template = require_path(&self.paths.message_setting_get, "message_setting_get")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_t(&path).await
    }
}
