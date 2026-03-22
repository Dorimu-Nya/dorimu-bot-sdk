use super::{OpenApiClient, OpenApiPaths, Result, TokenProvider, Value, render_path, require_path};

/// 发言设置查询接口。
#[derive(Clone)]
pub struct MessageSettingsApi<P> {
    pub(super) client: OpenApiClient<P>,
    pub(super) paths: OpenApiPaths,
}

impl<P> MessageSettingsApi<P>
where
    P: TokenProvider,
{
    pub async fn get(&self, guild_id: &str) -> Result<(http::StatusCode, Value)> {
        let template = require_path(&self.paths.message_setting_get, "message_setting_get")?;
        let path = render_path(&template, &[("guild_id", guild_id)])?;
        self.client.get_value(&path).await
    }
}


