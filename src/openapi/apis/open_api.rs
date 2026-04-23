use super::{
    AnnouncesApi, ApiPermissionsApi, C2cMessagesApi, ChannelPermissionsApi, ChannelsApi, ForumsApi,
    GroupMessagesApi, GuildsApi, InteractionsApi, MembersApi, MessageSettingsApi, MuteApi,
    OpenApiClient, OpenApiPaths, PinsApi, ReactionsApi, RolesApi, SchedulesApi, TokenProvider,
    UsersApi,
};

/// OpenAPI 业务入口，按领域返回各子 API。
#[derive(Clone)]
pub struct OpenApi<P> {
    client: OpenApiClient<P>,
    paths: OpenApiPaths,
}

impl<P> OpenApi<P>
where
    P: TokenProvider + Clone,
{
    pub fn new(client: OpenApiClient<P>, paths: OpenApiPaths) -> Self {
        Self { client, paths }
    }

    pub fn guilds(&self) -> GuildsApi<P> {
        GuildsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn channels(&self) -> ChannelsApi<P> {
        ChannelsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn members(&self) -> MembersApi<P> {
        MembersApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn interactions(&self) -> InteractionsApi<P> {
        InteractionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn reactions(&self) -> ReactionsApi<P> {
        ReactionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn announces(&self) -> AnnouncesApi<P> {
        AnnouncesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn api_permissions(&self) -> ApiPermissionsApi<P> {
        ApiPermissionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn channel_permissions(&self) -> ChannelPermissionsApi<P> {
        ChannelPermissionsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn pins(&self) -> PinsApi<P> {
        PinsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn roles(&self) -> RolesApi<P> {
        RolesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn schedules(&self) -> SchedulesApi<P> {
        SchedulesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn forums(&self) -> ForumsApi<P> {
        ForumsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn mute(&self) -> MuteApi<P> {
        MuteApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn message_settings(&self) -> MessageSettingsApi<P> {
        MessageSettingsApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn c2c_messages(&self) -> C2cMessagesApi<P> {
        C2cMessagesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn group_messages(&self) -> GroupMessagesApi<P> {
        GroupMessagesApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }

    pub fn users(&self) -> UsersApi<P> {
        UsersApi {
            client: self.client.clone(),
            paths: self.paths.clone(),
        }
    }
}
