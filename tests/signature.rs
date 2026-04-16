// use http::HeaderMap;
use qqbot_sdk::{
    sign_webhook_validation,
    // ReplayProtectionMode, SignatureConfig, SignatureVerifier,
};
// use std::time::{Duration,
// SystemTime, UNIX_EPOCH
// };

// #[test]
// fn verify_request_signature() {
//     let secret = "test_secret";
//     let timestamp = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_secs()
//         .to_string();
//     let body = b"{\"op\":0,\"d\":{}}";
//     let signature =
//         sign_webhook_validation(secret, &timestamp, std::str::from_utf8(body).unwrap()).unwrap();
//
//     let mut headers = HeaderMap::new();
//     headers.insert("x-signature-ed25519", signature.parse().unwrap());
//     headers.insert("x-signature-timestamp", timestamp.parse().unwrap());
//
//     let verifier =
//         SignatureVerifier::new(SignatureConfig::from_bot_secret(secret).unwrap()).unwrap();
//     verifier.verify(&headers, body).unwrap();
// }
//
// #[test]
// fn verify_request_signature_accepts_old_timestamp_in_monitor_mode() {
//     let secret = "test_secret";
//     let timestamp = "1700000000";
//     let body = b"{\"op\":0,\"d\":{}}";
//     let signature =
//         sign_webhook_validation(secret, timestamp, std::str::from_utf8(body).unwrap()).unwrap();
//
//     let mut headers = HeaderMap::new();
//     headers.insert("x-signature-ed25519", signature.parse().unwrap());
//     headers.insert("x-signature-timestamp", timestamp.parse().unwrap());
//
//     let config = SignatureConfig::from_bot_secret(secret)
//         .unwrap()
//         .with_replay_protection(ReplayProtectionMode::Monitor, Duration::from_secs(1));
//     let verifier = SignatureVerifier::new(config).unwrap();
//     verifier.verify(&headers, body).unwrap();
// }

// #[test]
// fn verify_request_signature_rejects_old_timestamp_in_enforce_mode() {
//     let secret = "test_secret";
//     let timestamp = "1700000000";
//     let body = b"{\"op\":0,\"d\":{}}";
//     let signature =
//         sign_webhook_validation(secret, timestamp, std::str::from_utf8(body).unwrap()).unwrap();
//
//     let mut headers = HeaderMap::new();
//     headers.insert("x-signature-ed25519", signature.parse().unwrap());
//     headers.insert("x-signature-timestamp", timestamp.parse().unwrap());
//
//     let config = SignatureConfig::from_bot_secret(secret)
//         .unwrap()
//         .with_replay_protection(ReplayProtectionMode::Enforce, Duration::from_secs(1));
//     let verifier = SignatureVerifier::new(config).unwrap();
//     let err = verifier.verify(&headers, body).unwrap_err();
//     assert!(matches!(err, qqbot_sdk::Error::InvalidTimestamp(_)));
// }
#[test]
fn verify_signature() {
    let secret = "DG5g3B4j9X2KOErG";

    let plain_token = "Arq0D5A61EgUu4OxUvOp";
    let event_ts = 1725442341.to_string();

    let expect = "87befc99c42c651b3aac0278e71ada338433ae26fcb24307bdc5ad38c1adc2d01bcfcadc0842edac85e85205028a1132afe09280305f13aa6909ffc2d652c706";

    let res = sign_webhook_validation(secret, &event_ts, plain_token);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expect);
}

#[test]
fn verify_signature_empty_secret_returns_error() {
    let res = sign_webhook_validation("", "1725442341", "Arq0D5A61EgUu4OxUvOp");
    assert!(matches!(res, Err(qqbot_sdk::Error::InvalidHeader(_))));
}

#[test]
fn verify_signature_changes_when_payload_changes() {
    let secret = "DG5g3B4j9X2KOErG";
    let sig1 = sign_webhook_validation(secret, "1725442341", "Arq0D5A61EgUu4OxUvOp").unwrap();
    let sig2 = sign_webhook_validation(secret, "1725442342", "Arq0D5A61EgUu4OxUvOp").unwrap();
    assert_ne!(sig1, sig2);
}
