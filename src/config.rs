/// 监听配置 
pub struct ListeningConfig {
    /// actix监听地址, 如0.0.0.0:3000
    pub bind_addr: String,
    /// webhook路径, 如/webhook
    pub webhook_path: String,
    //todo: 考虑加入其他自定义router...
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

/// 应用配置
pub struct AppConfig {
    /// 监听配置
    pub listening: ListeningConfig,
    /// qqbot票据配置
    pub credential: CredentialConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { listening: Default::default(), credential: Default::default() }
    }
}