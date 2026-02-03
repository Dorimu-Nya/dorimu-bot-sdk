use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct KvStore {
    path: PathBuf,
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl KvStore {
    pub async fn load(path: impl Into<PathBuf>) -> Self {
        // 简单 JSON 落盘的 KV 存储，适合小规模配置。
        let path = path.into();
        let mut map = HashMap::new();
        if let Ok(content) = tokio::fs::read_to_string(&path).await {
            if let Ok(parsed) = serde_json::from_str::<HashMap<String, String>>(&content) {
                map = parsed;
            }
        }
        Self {
            path,
            data: Arc::new(RwLock::new(map)),
        }
    }

    pub async fn get(&self, key: &str) -> std::io::Result<Option<String>> {
        let guard = self.data.read().await;
        Ok(guard.get(key).cloned())
    }

    pub async fn set(&self, key: &str, value: &str) -> std::io::Result<()> {
        let snapshot = {
            let mut guard = self.data.write().await;
            guard.insert(key.to_string(), value.to_string());
            guard.clone()
        };
        // 变更后落盘。
        self.flush(&snapshot).await
    }

    async fn flush(&self, data: &HashMap<String, String>) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let content = serde_json::to_string_pretty(data).unwrap_or_else(|_| "{}".to_string());
        tokio::fs::write(&self.path, content).await
    }
}
