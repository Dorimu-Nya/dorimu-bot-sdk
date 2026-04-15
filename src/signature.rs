use crate::{Error, Result};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
// use base64::Engine;
// use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
// use http::HeaderMap;
// use std::{
//     sync::Arc,
//     time::{Duration, SystemTime, UNIX_EPOCH},
// };
// use tracing::warn;

// sheip9(2026/4/10): 都是ai写的，反正能跑，可读性应该不是很重要，没问题就不管他了
// 使用exmaple:
//
// ```
// crate::signature::sign_webhook_validation(botsecret, &req.event_ts, &req.plain_token)?;
// ```

// /// 签名字符串的解码方式。
// #[derive(Debug, Clone)]
// pub enum SignatureEncoding {
//     /// 16 进制字符串。
//     Hex,
//     /// Base64 字符串。
//     Base64,
//     /// 自动识别：先尝试 Hex，再尝试 Base64。
//     Auto,
// }
//
// /// 抗重放校验模式。
// #[derive(Debug, Clone)]
// pub enum ReplayProtectionMode {
//     /// 关闭时间窗口校验，仅校验签名。
//     Off,
//     /// 仅记录异常时间戳，不拦截请求（推荐灰度阶段使用）。
//     Monitor,
//     /// 强制校验时间窗口，超窗请求直接拒绝。
//     Enforce,
// }
//
// /// 抗重放配置。
// #[derive(Debug, Clone)]
// pub struct ReplayProtectionConfig {
//     /// 校验模式。
//     pub mode: ReplayProtectionMode,
//     /// 允许的最大时间偏移（秒级时间戳与当前时间的绝对差）。
//     pub max_skew: Duration,
// }
//
// impl Default for ReplayProtectionConfig {
//     fn default() -> Self {
//         Self {
//             // 默认监控模式，保证对现网兼容。
//             mode: ReplayProtectionMode::Monitor,
//             // 默认允许 10 分钟偏移，降低网络抖动和平台重试带来的误判。
//             max_skew: Duration::from_secs(10 * 60),
//         }
//     }
// }
//
// /// Webhook 签名验证配置。
// #[derive(Clone)]
// pub struct SignatureConfig {
//     /// Ed25519 公钥（32 字节）。
//     pub public_key: Vec<u8>,
//     /// 签名头名称。
//     pub signature_header: http::header::HeaderName,
//     /// 时间戳头名称。
//     pub timestamp_header: http::header::HeaderName,
//     /// 签名编码方式。
//     pub encoding: SignatureEncoding,
//     /// 构造签名明文的函数，默认是 `timestamp + body`。
//     #[allow(clippy::type_complexity)]
//     pub payload_builder: Arc<dyn Fn(&str, &[u8]) -> Vec<u8> + Send + Sync>,
//     /// 抗重放配置。
//     pub replay_protection: ReplayProtectionConfig,
// }
//
// impl SignatureConfig {
//     /// 使用公钥创建默认签名配置。
//     pub fn new(public_key: Vec<u8>) -> Self {
//         Self {
//             public_key,
//             signature_header: http::header::HeaderName::from_static("x-signature-ed25519"),
//             timestamp_header: http::header::HeaderName::from_static("x-signature-timestamp"),
//             encoding: SignatureEncoding::Auto,
//             payload_builder: Arc::new(|timestamp, body| {
//                 let mut buf = Vec::with_capacity(timestamp.len() + body.len());
//                 buf.extend_from_slice(timestamp.as_bytes());
//                 buf.extend_from_slice(body);
//                 buf
//             }),
//             replay_protection: ReplayProtectionConfig::default(),
//         }
//     }
//
//     pub fn with_headers(
//         mut self,
//         signature: http::header::HeaderName,
//         timestamp: http::header::HeaderName,
//     ) -> Self {
//         // 允许接入方自定义平台头名称。
//         self.signature_header = signature;
//         self.timestamp_header = timestamp;
//         self
//     }
//
//     /// 设置签名编码方式。
//     pub fn with_encoding(mut self, encoding: SignatureEncoding) -> Self {
//         self.encoding = encoding;
//         self
//     }
//
//     /// 快捷设置抗重放策略。
//     pub fn with_replay_protection(
//         mut self,
//         mode: ReplayProtectionMode,
//         max_skew: Duration,
//     ) -> Self {
//         self.replay_protection = ReplayProtectionConfig { mode, max_skew };
//         self
//     }
//
//     pub fn with_replay_protection_config(
//         mut self,
//         replay_protection: ReplayProtectionConfig,
//     ) -> Self {
//         // 用完整配置覆盖默认值，便于统一管理。
//         self.replay_protection = replay_protection;
//         self
//     }
//
//     /// 使用 Bot Secret 推导公钥并创建签名配置。
//     pub fn from_bot_secret(bot_secret: &str) -> Result<Self> {
//         let public_key = public_key_from_bot_secret(bot_secret)?;
//         Ok(SignatureConfig::new(public_key))
//     }
// }
//
// /// Webhook 签名验证器。
// #[derive(Clone)]
// pub struct SignatureVerifier {
//     config: SignatureConfig,
//     key: VerifyingKey,
// }
//
// impl SignatureVerifier {
//     /// 构建签名验证器。
//     pub fn new(config: SignatureConfig) -> Result<Self> {
//         let key_bytes: [u8; 32] = config
//             .public_key
//             .as_slice()
//             .try_into()
//             .map_err(|_| Error::InvalidHeader("public key must be 32 bytes".to_string()))?;
//         let key = VerifyingKey::from_bytes(&key_bytes)
//             .map_err(|_| Error::InvalidHeader("invalid public key".to_string()))?;
//         Ok(Self { config, key })
//     }
//
//     /// 使用 Bot Secret 构建签名验证器。
//     pub fn from_bot_secret(bot_secret: &str) -> Result<Self> {
//         SignatureConfig::from_bot_secret(bot_secret).and_then(SignatureVerifier::new)
//     }
//
//     /// 校验请求头与请求体签名。
//     pub fn verify(&self, headers: &HeaderMap, body: &[u8]) -> Result<()> {
//         let sig = headers
//             .get(&self.config.signature_header)
//             .ok_or(Error::MissingHeader("signature"))?
//             .to_str()
//             .map_err(|_| Error::InvalidHeader("signature header is not valid utf-8".to_string()))?;
//
//         let timestamp = headers
//             .get(&self.config.timestamp_header)
//             .ok_or(Error::MissingHeader("timestamp"))?
//             .to_str()
//             .map_err(|_| Error::InvalidHeader("timestamp header is not valid utf-8".to_string()))?;
//
//         // 先做时间窗口判断，再做签名校验。
//         self.validate_timestamp(timestamp)?;
//
//         let signature_bytes = decode_signature(sig, &self.config.encoding)?;
//         if signature_bytes.len() != 64 {
//             return Err(Error::InvalidHeader(
//                 "signature must be 64 bytes".to_string(),
//             ));
//         }
//
//         let mut sig_arr = [0u8; 64];
//         sig_arr.copy_from_slice(&signature_bytes);
//         let signature = Signature::from_bytes(&sig_arr);
//
//         let payload = (self.config.payload_builder)(timestamp, body);
//         self.key
//             .verify(&payload, &signature)
//             .map_err(|_| Error::InvalidSignature)
//     }
//
//     /// 校验时间戳是否落在允许窗口，用于降低重放风险。
//     fn validate_timestamp(&self, timestamp: &str) -> Result<()> {
//         let protection = &self.config.replay_protection;
//         if matches!(protection.mode, ReplayProtectionMode::Off) {
//             return Ok(());
//         }
//
//         let ts = match timestamp.parse::<i64>() {
//             Ok(v) => v,
//             Err(_) => {
//                 return match protection.mode {
//                     ReplayProtectionMode::Monitor => {
//                         // 监控模式只记录，不影响线上流量。
//                         warn!(timestamp = %timestamp, "signature timestamp is not a unix seconds number");
//                         Ok(())
//                     }
//                     ReplayProtectionMode::Enforce => Err(Error::InvalidTimestamp(
//                         "timestamp header must be unix seconds".to_string(),
//                     )),
//                     ReplayProtectionMode::Off => Ok(()),
//                 };
//             }
//         };
//
//         let now = SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .map_err(|_| Error::InvalidTimestamp("system clock is before unix epoch".to_string()))?
//             .as_secs() as i64;
//
//         let skew = now.saturating_sub(ts).abs();
//         let max_skew_secs = protection.max_skew.as_secs() as i64;
//         if skew <= max_skew_secs {
//             return Ok(());
//         }
//
//         match protection.mode {
//             ReplayProtectionMode::Monitor => {
//                 // 灰度阶段输出告警，便于评估切 Enforce 的风险。
//                 warn!(
//                     now = now,
//                     timestamp = ts,
//                     skew_seconds = skew,
//                     max_skew_seconds = max_skew_secs,
//                     "signature timestamp is outside replay protection window"
//                 );
//                 Ok(())
//             }
//             ReplayProtectionMode::Enforce => Err(Error::InvalidTimestamp(format!(
//                 "timestamp is outside allowed skew window (max {}s)",
//                 max_skew_secs
//             ))),
//             ReplayProtectionMode::Off => Ok(()),
//         }
//     }
// }
//
// /// 解析签名字符串。
// fn decode_signature(sig: &str, encoding: &SignatureEncoding) -> Result<Vec<u8>> {
//     match encoding {
//         SignatureEncoding::Hex => hex::decode(sig)
//             .map_err(|_| Error::InvalidHeader("signature hex decode failed".to_string())),
//         SignatureEncoding::Base64 => base64::engine::general_purpose::STANDARD
//             .decode(sig)
//             .map_err(|_| Error::InvalidHeader("signature base64 decode failed".to_string())),
//         SignatureEncoding::Auto => {
//             if let Ok(bytes) = hex::decode(sig) {
//                 return Ok(bytes);
//             }
//             base64::engine::general_purpose::STANDARD
//                 .decode(sig)
//                 .map_err(|_| Error::InvalidHeader("signature decode failed".to_string()))
//         }
//     }
// }

// sheip9 (2026/4/16): 看不出来上面那部分代码有鸟用

/// 生成 webhook 校验回包签名（`event_ts + plain_token`）。
pub fn sign_webhook_validation(
    bot_secret: &str,
    event_ts: &str,
    plain_token: &str,
) -> Result<String> {
    let signing_key = signing_key_from_bot_secret(bot_secret)?;
    let mut payload = Vec::with_capacity(event_ts.len() + plain_token.len());
    payload.extend_from_slice(event_ts.as_bytes());
    payload.extend_from_slice(plain_token.as_bytes());
    let signature = signing_key.sign(&payload);
    Ok(hex::encode(signature.to_bytes()))
}

/// 使用 Bot Secret 推导 Ed25519 公钥。
pub fn public_key_from_bot_secret(bot_secret: &str) -> Result<Vec<u8>> {
    let signing_key = signing_key_from_bot_secret(bot_secret)?;
    let verifying_key = VerifyingKey::from(&signing_key);
    Ok(verifying_key.to_bytes().to_vec())
}

/// 使用 Bot Secret 推导 Ed25519 私钥种子。
fn signing_key_from_bot_secret(bot_secret: &str) -> Result<SigningKey> {
    if bot_secret.is_empty() {
        return Err(Error::InvalidHeader("bot secret is empty".to_string()));
    }
    // 与官方实现保持兼容：通过重复 secret 填充到 32 字节种子。
    let mut seed = Vec::new();
    while seed.len() < 32 {
        seed.extend_from_slice(bot_secret.as_bytes());
    }
    seed.truncate(32);
    let mut seed_arr = [0u8; 32];
    seed_arr.copy_from_slice(&seed);
    Ok(SigningKey::from_bytes(&seed_arr))
}
