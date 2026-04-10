use crate::config::AppConfig;
use crate::HttpTokenProvider;
use crate::OpenApi;
use crate::OpenApiClient;
use crate::OpenApiConfig;
use crate::OpenApiPaths;
use crate::TokenManager;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct App {
    pub config: AppConfig,
    api_client: Arc<OpenApi<HttpTokenProvider>>,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let token_provider = HttpTokenProvider::from_env_or_official(
            &config.credential.app_id,
            &config.credential.secret,
        );
        let token_manager = TokenManager::new(token_provider, Duration::from_secs(120));
        let client = OpenApiClient::new(token_manager, OpenApiConfig::from_env_or_official());
        let api = OpenApi::new(client, OpenApiPaths::official_defaults());

        Self {
            config,
            api_client: Arc::new(api),
        }
    }

    pub fn get_api_client(&self) -> Arc<OpenApi<HttpTokenProvider>> {
        Arc::clone(&self.api_client)
    }
}
