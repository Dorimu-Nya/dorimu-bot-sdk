use super::{render_path, require_path, OpenApiClient, OpenApiPaths, Result, TokenProvider};
use crate::openapi::models::MessageSetting;

/// 发言设置查询接口。
#[derive(Clone)]
pub struct MessageSettingsApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 消息设置接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> MessageSettingsApi<P>
where
    P: TokenProvider,
{
    /// 获取指定频道的消息设置。
    pub async fn get(&self, guild_id: &str) -> Result<(http::StatusCode, MessageSetting)> {
        let template = require_path(&self.paths.message_setting_get, "message_setting_get")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_t(&path).await
    }
}
