use serde::{Deserialize, Serialize};

/// GET /users/@me/guilds 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserGuildsQuery {
    /// 读此 guild id 之前的数据。
    #[serde(default)]
    pub before: Option<String>,
    /// 读此 guild id 之后的数据。
    #[serde(default)]
    pub after: Option<String>,
    /// 每次拉取多少条数据，默认 100, 最大 100。
    #[serde(default)]
    pub limit: Option<u32>,
}
