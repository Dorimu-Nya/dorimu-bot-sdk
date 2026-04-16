use super::ContextStore;
use qqbot_sdk::Context;
use std::any::Any;

/// 监听配置
#[derive(Clone)]
pub struct ListeningConfig {
    /// actix监听地址, 如0.0.0.0:3000
    pub bind_addr: String,
    /// webhook路径, 如/webhook
    pub webhook_path: String,
}

impl Default for ListeningConfig {
    fn default() -> Self {
        Self {
            bind_addr: String::from("0.0.0.0:3000"),
            webhook_path: String::from("/webhook"),
        }
    }
}

/// qqbot官网下发的票据
#[derive(Clone)]
pub struct CredentialConfig {
    pub app_id: String,
    pub secret: String,
}

impl Default for CredentialConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            secret: String::new(),
        }
    }
}

#[derive(Clone)]
pub struct SandboxConfig {
    // TODO: 这里应该是放一些沙箱里的openid什么的或者拦截器？
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {}
    }
}

/// qqbot api地址覆盖配置
#[derive(Clone)]
pub struct QQApiOverrides {
    pub prod_url_override: Option<String>,
    pub sandbox_url_override: Option<String>,
}

impl Default for QQApiOverrides {
    fn default() -> Self {
        Self {
            prod_url_override: None,
            sandbox_url_override: None,
        }
    }
}

/// 应用配置
pub struct AppConfig {
    /// 监听配置
    pub listening: ListeningConfig,
    /// qqbot票据配置
    pub credential: CredentialConfig,
    /// 沙箱配置
    pub sandbox_config: SandboxConfig,
    /// api地址覆写
    pub api_overrides: QQApiOverrides,
    /// 启动时检查，包括上下文检查，指令是否重复等
    pub ignore_checking: bool,
    ///
    pub contexts: Vec<Box<dyn Fn(&ContextStore) -> Option<&str> + Send + Sync>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            listening: Default::default(),
            credential: Default::default(),
            sandbox_config: Default::default(),
            api_overrides: Default::default(),
            ignore_checking: false,
            contexts: vec![],
        }
    }
}

impl AppConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_addr(mut self, bind: &str) -> Self {
        self.listening.bind_addr = bind.to_string();
        self
    }

    pub fn webhook_path(mut self, path: &str) -> Self {
        self.listening.webhook_path = path.to_string();
        self
    }

    pub fn credential(mut self, credential: CredentialConfig) -> Self {
        self.credential = credential;
        self
    }

    pub fn prod_url_override(mut self, api: &str) -> Self {
        self.api_overrides.prod_url_override = Some(api.to_string());
        self
    }

    pub fn with_context<T: Any + Send + Sync + 'static>(mut self, context: Context<T>) -> Self {
        self.contexts.push(Box::new(move |store: &ContextStore| {
            store.insert_arc(context.as_arc())
        }));
        self
    }
}
