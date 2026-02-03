use http::HeaderMap;
use qqbot_sdk::{sign_webhook_validation, SignatureConfig, SignatureVerifier};

#[test]
fn verify_request_signature() {
    let secret = "test_secret";
    let timestamp = "1700000000";
    let body = b"{\"op\":0,\"d\":{}}";
    let signature = sign_webhook_validation(secret, timestamp, std::str::from_utf8(body).unwrap()).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        "x-signature-ed25519",
        signature.parse().unwrap(),
    );
    headers.insert("x-signature-timestamp", timestamp.parse().unwrap());

    let verifier = SignatureVerifier::new(SignatureConfig::from_bot_secret(secret).unwrap()).unwrap();
    verifier.verify(&headers, body).unwrap();
}
