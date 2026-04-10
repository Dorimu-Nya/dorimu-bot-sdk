use crate::config::AppConfig;

#[derive(Clone)]
pub struct App {
    pub config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}