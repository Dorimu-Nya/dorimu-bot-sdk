use http::HeaderMap;
use qqbot_sdk::{sign_webhook_validation, webhook_validation_hook};
use serde_json::json;

#[test]
fn webhook_validation_hook_returns_signature() {
    let secret = "DG5g3B4j9X2KOErG";
    let payload = json!({
        "op": 13,
        "d": {
            "plain_token": "Arq0D5A61EgUu4OxUvOp",
            "event_ts": "1725442341"
        }
    });

    let hook = webhook_validation_hook(secret);
    let resp = hook(&HeaderMap::new(), b"", &payload).unwrap().unwrap();

    assert_eq!(resp.status, http::StatusCode::OK);
    let value: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(value["plain_token"], "Arq0D5A61EgUu4OxUvOp");
    let expected = sign_webhook_validation(secret, "1725442341", "Arq0D5A61EgUu4OxUvOp").unwrap();
    assert_eq!(value["signature"], expected);
}
