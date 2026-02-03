use std::env;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub webhook_path: String,
    pub bot_secret: String,
    pub app_id: String,
    pub client_secret: String,
    pub cmd_prefix: String,
    pub data_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        // 保留默认值，便于本地快速启动。
        let host = env::var("QQ_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("QQ_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(8080);
        let webhook_path = env::var("QQ_WEBHOOK_PATH").unwrap_or_else(|_| "/webhook".to_string());
        let bot_secret = env::var("QQ_BOT_SECRET").expect("QQ_BOT_SECRET missing");
        let app_id = env::var("QQ_APP_ID").expect("QQ_APP_ID missing");
        let client_secret = env::var("QQ_CLIENT_SECRET").expect("QQ_CLIENT_SECRET missing");
        let cmd_prefix = env::var("BOT_CMD_PREFIX").unwrap_or_else(|_| "/".to_string());
        let data_path = env::var("BOT_DATA_PATH").unwrap_or_else(|_| "data/kv.json".to_string());

        Self {
            host,
            port,
            webhook_path,
            bot_secret,
            app_id,
            client_secret,
            cmd_prefix,
            data_path,
        }
    }
}
