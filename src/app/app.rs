use super::AppConfig;
use super::ContextStore;
use crate::openapi::{
    HttpTokenProvider, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths, TokenManager,
};
use std::sync::Arc;
use std::time::Duration;
use crate::CredentialConfig;
use super::commands::store::CommandsStore;

#[derive(Clone)]
pub struct App {
    pub credential: CredentialConfig,
    prod_api_client: Arc<OpenApi<HttpTokenProvider>>,
    pub dependency_container: ContextStore,
    pub commands: CommandsStore,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        // api 客户端初始化
        let token_provider = HttpTokenProvider::from_env_or_official(
            &config.credential.app_id,
            &config.credential.secret,
        );
        let token_manager = TokenManager::new(token_provider, Duration::from_secs(120));
        let mut openapi_config = OpenApiConfig::official();
        if let Some(url) = &config.api_overrides.prod_url_override {
            openapi_config.base_url = url.clone();
        }
        let client = OpenApiClient::new(token_manager, openapi_config);
        let api = OpenApi::new(client, OpenApiPaths::official_defaults());
        // api 客户端初始化 end

        // 初始化ioc
        let container = ContextStore::new();

        for register in &config.contexts {
            register(&container);
        }

        Self {
            credential: config.credential,
            prod_api_client: Arc::new(api),
            dependency_container: container,
            commands: CommandsStore::new(),
        }
    }

    pub fn get_prod_client(&self) -> Arc<OpenApi<HttpTokenProvider>> {
        Arc::clone(&self.prod_api_client)
    }

}
