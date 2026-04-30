use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 操作按钮回包状态码。
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum InteractionAckCode {
    /// 0 成功。
    Success = 0,
    /// 1 操作失败。
    Failed = 1,
    /// 2 操作频繁。
    TooFrequent = 2,
    /// 3 重复操作。
    Duplicated = 3,
    /// 4 没有权限。
    NoPermission = 4,
    /// 5 仅管理员操作。
    AdminOnly = 5,
}

/// PUT /interactions/{interaction_id} 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionAckRequest {
    /// 0 成功 1 操作失败 2 操作频繁 3 重复操作 4 没有权限 5 仅管理员操作。
    pub code: InteractionAckCode,
}
