use super::commands::store::CommandsStore;
use super::AppConfig;
use super::ContextStore;
use crate::openapi::{
    HttpTokenProvider, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths, TokenManager,
};
use crate::{CommandDef, CredentialConfig};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct App {
    /// 票据配置
    pub credential: CredentialConfig,
    /// 生产环境的 api 客户端
    prod_api_client: Arc<OpenApi<HttpTokenProvider>>,
    /// 依赖容器
    pub dependency_container: ContextStore,
    /// 命令函数容器
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
            let o = register(&container);
            if !config.ignore_checking {
                if let Some(c) = o {
                    panic!("Context {:?} 重复传入！", c);
                }
            }
        }

        let mut commands = HashMap::new();

        #[cfg(feature = "macros")]
        inventory::iter::<CommandDef>.into_iter().for_each(|x| {
            let o = commands.insert(x.prefix, x.handler);
            if !config.ignore_checking {
                if let Some(_) = o {
                    panic!("Command {:?} 重复传入！", x.prefix);
                }
            }
        });

        let commands = CommandsStore::new(commands);

        Self {
            credential: config.credential,
            prod_api_client: Arc::new(api),
            dependency_container: container,
            commands,
        }
    }

    /// 获取生产环境的 api 客户端。
    pub fn get_prod_client(&self) -> Arc<OpenApi<HttpTokenProvider>> {
        Arc::clone(&self.prod_api_client)
    }
}
