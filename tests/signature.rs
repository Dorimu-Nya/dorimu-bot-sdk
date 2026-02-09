use http::HeaderMap;
use qqbot_sdk::{
    sign_webhook_validation, ReplayProtectionMode, SignatureConfig, SignatureVerifier,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[test]
fn verify_request_signature() {
    let secret = "test_secret";
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    let body = b"{\"op\":0,\"d\":{}}";
    let signature =
        sign_webhook_validation(secret, &timestamp, std::str::from_utf8(body).unwrap()).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("x-signature-ed25519", signature.parse().unwrap());
    headers.insert("x-signature-timestamp", timestamp.parse().unwrap());

    let verifier =
        SignatureVerifier::new(SignatureConfig::from_bot_secret(secret).unwrap()).unwrap();
    verifier.verify(&headers, body).unwrap();
}

#[test]
fn verify_request_signature_accepts_old_timestamp_in_monitor_mode() {
    let secret = "test_secret";
    let timestamp = "1700000000";
    let body = b"{\"op\":0,\"d\":{}}";
    let signature =
        sign_webhook_validation(secret, timestamp, std::str::from_utf8(body).unwrap()).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("x-signature-ed25519", signature.parse().unwrap());
    headers.insert("x-signature-timestamp", timestamp.parse().unwrap());

    let config = SignatureConfig::from_bot_secret(secret)
        .unwrap()
        .with_replay_protection(ReplayProtectionMode::Monitor, Duration::from_secs(1));
    let verifier = SignatureVerifier::new(config).unwrap();
    verifier.verify(&headers, body).unwrap();
}

#[test]
fn verify_request_signature_rejects_old_timestamp_in_enforce_mode() {
    let secret = "test_secret";
    let timestamp = "1700000000";
    let body = b"{\"op\":0,\"d\":{}}";
    let signature =
        sign_webhook_validation(secret, timestamp, std::str::from_utf8(body).unwrap()).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("x-signature-ed25519", signature.parse().unwrap());
    headers.insert("x-signature-timestamp", timestamp.parse().unwrap());

    let config = SignatureConfig::from_bot_secret(secret)
        .unwrap()
        .with_replay_protection(ReplayProtectionMode::Enforce, Duration::from_secs(1));
    let verifier = SignatureVerifier::new(config).unwrap();
    let err = verifier.verify(&headers, body).unwrap_err();
    assert!(matches!(err, qqbot_sdk::Error::InvalidTimestamp(_)));
}
