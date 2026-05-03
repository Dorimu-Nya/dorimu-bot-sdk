use super::{
    AnnouncesApi, ApiPermissionsApi, AudioApi, C2cMessagesApi, ChannelMessagesApi,
    ChannelPermissionsApi, ChannelsApi, DmsMessagesApi, ForumsApi, GroupMessagesApi, GuildsApi,
    InteractionsApi, MediaApi, MembersApi, MessageSettingsApi, MuteApi, OpenApiClient,
    OpenApiPaths, PinsApi, ReactionsApi, RolesApi, SchedulesApi, TokenProvider, UsersApi,
};

/// OpenAPI 业务入口，按领域返回各子 API。
#[derive(Clone)]
pub struct OpenApi<P> {
    /// 共享的 OpenAPI HTTP 客户端。
    client: OpenApiClient<P>,
    /// 当前入口使用的 OpenAPI 路径模板集合。
    paths: OpenApiPaths,
}

impl<P> OpenApi<P>
where
    P: TokenProvider + Clone,
{
    /// 创建 OpenAPI 业务入口。
    pub fn new(client: OpenApiClient<P>, paths: OpenApiPaths) -> Self {
        Self { client, paths }
    }

    /// 获取频道（Guild）相关 API。
    pub fn guilds(&self) -> GuildsApi<P> {
        GuildsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取子频道（Channel）相关 API。
    pub fn channels(&self) -> ChannelsApi<P> {
        ChannelsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取频道成员相关 API。
    pub fn members(&self) -> MembersApi<P> {
        MembersApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取互动事件回调相关 API。
    pub fn interactions(&self) -> InteractionsApi<P> {
        InteractionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取表情回应相关 API。
    pub fn reactions(&self) -> ReactionsApi<P> {
        ReactionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取频道公告相关 API。
    pub fn announces(&self) -> AnnouncesApi<P> {
        AnnouncesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取 API 权限申请相关 API。
    pub fn api_permissions(&self) -> ApiPermissionsApi<P> {
        ApiPermissionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取子频道权限相关 API。
    pub fn channel_permissions(&self) -> ChannelPermissionsApi<P> {
        ChannelPermissionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取子频道消息相关 API。
    pub fn channel_messages(&self) -> ChannelMessagesApi<P> {
        ChannelMessagesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取音频、麦克风和语音成员相关 API。
    pub fn audio(&self) -> AudioApi<P> {
        AudioApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取精华消息相关 API。
    pub fn pins(&self) -> PinsApi<P> {
        PinsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取频道身份组相关 API。
    pub fn roles(&self) -> RolesApi<P> {
        RolesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取日程相关 API。
    pub fn schedules(&self) -> SchedulesApi<P> {
        SchedulesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取论坛帖子相关 API。
    pub fn forums(&self) -> ForumsApi<P> {
        ForumsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取禁言相关 API。
    pub fn mute(&self) -> MuteApi<P> {
        MuteApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取消息设置相关 API。
    pub fn message_settings(&self) -> MessageSettingsApi<P> {
        MessageSettingsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取 C2C 单聊消息相关 API。
    pub fn c2c_messages(&self) -> C2cMessagesApi<P> {
        C2cMessagesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取群聊消息和群成员相关 API。
    pub fn group_messages(&self) -> GroupMessagesApi<P> {
        GroupMessagesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取私信消息相关 API。
    pub fn dms_messages(&self) -> DmsMessagesApi<P> {
        DmsMessagesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取富媒体上传相关 API。
    pub fn media(&self) -> MediaApi<P> {
        MediaApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    /// 获取当前用户、频道列表和私信创建相关 API。
    pub fn users(&self) -> UsersApi<P> {
        UsersApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }
}
