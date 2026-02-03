use crate::{Error, Result};
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use http::HeaderMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum SignatureEncoding {
    Hex,
    Base64,
    Auto,
}

#[derive(Clone)]
pub struct SignatureConfig {
    pub public_key: Vec<u8>,
    pub signature_header: http::header::HeaderName,
    pub timestamp_header: http::header::HeaderName,
    pub encoding: SignatureEncoding,
    pub payload_builder: Arc<dyn Fn(&str, &[u8]) -> Vec<u8> + Send + Sync>,
}

impl SignatureConfig {
    pub fn new(public_key: Vec<u8>) -> Self {
        Self {
            public_key,
            signature_header: http::header::HeaderName::from_static("x-signature-ed25519"),
            timestamp_header: http::header::HeaderName::from_static("x-signature-timestamp"),
            encoding: SignatureEncoding::Auto,
            payload_builder: Arc::new(|timestamp, body| {
                let mut buf = Vec::with_capacity(timestamp.len() + body.len());
                buf.extend_from_slice(timestamp.as_bytes());
                buf.extend_from_slice(body);
                buf
            }),
        }
    }

    pub fn with_headers(mut self, signature: http::header::HeaderName, timestamp: http::header::HeaderName) -> Self {
        self.signature_header = signature;
        self.timestamp_header = timestamp;
        self
    }

    pub fn with_encoding(mut self, encoding: SignatureEncoding) -> Self {
        self.encoding = encoding;
        self
    }

    pub fn from_bot_secret(bot_secret: &str) -> Result<Self> {
        let public_key = public_key_from_bot_secret(bot_secret)?;
        Ok(SignatureConfig::new(public_key))
    }
}

#[derive(Clone)]
pub struct SignatureVerifier {
    config: SignatureConfig,
    key: VerifyingKey,
}

impl SignatureVerifier {
    pub fn new(config: SignatureConfig) -> Result<Self> {
        let key_bytes: [u8; 32] = config
            .public_key
            .as_slice()
            .try_into()
            .map_err(|_| Error::InvalidHeader("public key must be 32 bytes".to_string()))?;
        let key = VerifyingKey::from_bytes(&key_bytes).map_err(|_| Error::InvalidHeader("invalid public key".to_string()))?;
        Ok(Self { config, key })
    }

    pub fn from_bot_secret(bot_secret: &str) -> Result<Self> {
        SignatureConfig::from_bot_secret(bot_secret).and_then(SignatureVerifier::new)
    }

    pub fn verify(&self, headers: &HeaderMap, body: &[u8]) -> Result<()> {
        let sig = headers
            .get(&self.config.signature_header)
            .ok_or(Error::MissingHeader("signature"))?
            .to_str()
            .map_err(|_| Error::InvalidHeader("signature header is not valid utf-8".to_string()))?;

        let timestamp = headers
            .get(&self.config.timestamp_header)
            .ok_or(Error::MissingHeader("timestamp"))?
            .to_str()
            .map_err(|_| Error::InvalidHeader("timestamp header is not valid utf-8".to_string()))?;

        let signature_bytes = decode_signature(sig, &self.config.encoding)?;
        if signature_bytes.len() != 64 {
            return Err(Error::InvalidHeader("signature must be 64 bytes".to_string()));
        }

        let mut sig_arr = [0u8; 64];
        sig_arr.copy_from_slice(&signature_bytes);
        let signature = Signature::from_bytes(&sig_arr);

        let payload = (self.config.payload_builder)(timestamp, body);
        self.key
            .verify(&payload, &signature)
            .map_err(|_| Error::InvalidSignature)
    }
}

fn decode_signature(sig: &str, encoding: &SignatureEncoding) -> Result<Vec<u8>> {
    match encoding {
        SignatureEncoding::Hex => hex::decode(sig).map_err(|_| Error::InvalidHeader("signature hex decode failed".to_string())),
        SignatureEncoding::Base64 => {
            base64::engine::general_purpose::STANDARD
                .decode(sig)
                .map_err(|_| Error::InvalidHeader("signature base64 decode failed".to_string()))
        }
        SignatureEncoding::Auto => {
            if let Ok(bytes) = hex::decode(sig) {
                return Ok(bytes);
            }
            base64::engine::general_purpose::STANDARD
                .decode(sig)
                .map_err(|_| Error::InvalidHeader("signature decode failed".to_string()))
        }
    }
}

pub fn sign_webhook_validation(bot_secret: &str, event_ts: &str, plain_token: &str) -> Result<String> {
    let signing_key = signing_key_from_bot_secret(bot_secret)?;
    let mut payload = Vec::with_capacity(event_ts.len() + plain_token.len());
    payload.extend_from_slice(event_ts.as_bytes());
    payload.extend_from_slice(plain_token.as_bytes());
    let signature = signing_key.sign(&payload);
    Ok(hex::encode(signature.to_bytes()))
}

pub fn public_key_from_bot_secret(bot_secret: &str) -> Result<Vec<u8>> {
    let signing_key = signing_key_from_bot_secret(bot_secret)?;
    let verifying_key = VerifyingKey::from(&signing_key);
    Ok(verifying_key.to_bytes().to_vec())
}

fn signing_key_from_bot_secret(bot_secret: &str) -> Result<SigningKey> {
    if bot_secret.is_empty() {
        return Err(Error::InvalidHeader("bot secret is empty".to_string()));
    }
    let mut seed = Vec::new();
    while seed.len() < 32 {
        seed.extend_from_slice(bot_secret.as_bytes());
    }
    seed.truncate(32);
    let mut seed_arr = [0u8; 32];
    seed_arr.copy_from_slice(&seed);
    Ok(SigningKey::from_bytes(&seed_arr))
}
