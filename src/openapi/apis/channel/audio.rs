use super::{
    render_path, require_path, Method, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};
use crate::openapi::models::{AudioControlRequest, Member};

/// 音频、麦克风和语音成员相关接口。
#[derive(Clone)]
pub struct AudioApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    /// 音频和语音接口使用的路径模板。
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> AudioApi<P>
where
    P: TokenProvider,
{
    /// 对指定子频道执行音频播放控制。
    pub async fn control(
        &self,
        channel_id: &str,
        body: &AudioControlRequest,
    ) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.channel_audio_control, "channel_audio_control")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        let resp = self
            .client
            .request_json_with(Method::POST, &path, Some(body))
            .await?;
        Ok(resp.status())
    }

    /// 将机器人加入指定语音子频道麦克风。
    pub async fn put_mic(&self, channel_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.channel_mic, "channel_mic")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        let resp = self.client.request_json(Method::PUT, &path, None).await?;
        Ok(resp.status())
    }

    /// 将机器人移出指定语音子频道麦克风。
    pub async fn delete_mic(&self, channel_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.channel_mic, "channel_mic")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        let resp = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(resp.status())
    }

    /// 获取指定语音子频道中的成员列表。
    pub async fn voice_members(&self, channel_id: &str) -> Result<(http::StatusCode, Vec<Member>)> {
        let template = require_path(&self.paths.channel_voice_members, "channel_voice_members")?;
        let path = render_path(&template, &[("channel_id", channel_id)])?;
        self.client.get_t(&path).await
    }
}
