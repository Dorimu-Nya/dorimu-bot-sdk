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
    prod_api_client: Arc<OpenApi<HttpTokenProvider>>,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
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

        Self {
            config,
            prod_api_client: Arc::new(api),
        }
    }

    pub fn get_prod_client(&self) -> Arc<OpenApi<HttpTokenProvider>> {
        Arc::clone(&self.prod_api_client)
    }
}
