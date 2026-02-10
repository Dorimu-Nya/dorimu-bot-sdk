use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

/// KV 存储容量限制，防止恶意写入耗尽内存。
const MAX_ENTRIES: usize = 1000;
const MAX_KEY_LEN: usize = 128;
const MAX_VALUE_LEN: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    KeyTooLong,
    ValueTooLong,
    CapacityExceeded,
    Io(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyTooLong => write!(f, "key 长度超过 {} 字节限制", MAX_KEY_LEN),
            Self::ValueTooLong => write!(f, "value 长度超过 {} 字节限制", MAX_VALUE_LEN),
            Self::CapacityExceeded => write!(f, "存储条目数已达上限 {}", MAX_ENTRIES),
            Self::Io(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<std::io::Error> for StoreError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

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

    pub async fn get(&self, key: &str) -> Result<Option<String>, StoreError> {
        let guard = self.data.read().await;
        Ok(guard.get(key).cloned())
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), StoreError> {
        if key.len() > MAX_KEY_LEN {
            return Err(StoreError::KeyTooLong);
        }
        if value.len() > MAX_VALUE_LEN {
            return Err(StoreError::ValueTooLong);
        }
        let snapshot = {
            let mut guard = self.data.write().await;
            // 新增 key 时检查容量上限（更新已有 key 不受限制）。
            if !guard.contains_key(key) && guard.len() >= MAX_ENTRIES {
                return Err(StoreError::CapacityExceeded);
            }
            guard.insert(key.to_string(), value.to_string());
            guard.clone()
        };
        // 变更后落盘。
        self.flush(&snapshot).await?;
        Ok(())
    }

    async fn flush(&self, data: &HashMap<String, String>) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let content = serde_json::to_string_pretty(data).unwrap_or_else(|_| "{}".to_string());
        tokio::fs::write(&self.path, content).await
    }
}
