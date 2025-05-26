use didcomm_module::{JsonWebKey, KeySharingMessageBuilder, MessageBuilderError};
use serde_json::json;

#[test]
pub fn key_sharing() {
    let msg = KeySharingMessageBuilder::new()
        .id("message-id".to_string())
        .add_key(key1())
        .add_key(key2())
        .build()
        .unwrap();

    let expected = json!({
    "body": {
        "keys": [
        {
            "crv": "P-256",
            "d": "0002",
            "kid": "<key fingerprint>",
            "kty": "EC",
            "use": "enc",
            "x": "0000",
            "y": "0001"
        },
        {
            "crv": "P-256",
            "d": "0004",
            "kid": "<key fingerprint>",
            "kty": "EC",
            "use": "enc",
            "x": "0002",
            "y": "0003"
        },
        ]
    },
    "id": "message-id",
    "typ": "application/didcomm-plain+json",
    "type": "https://didcomm.org/key-sharing/1.0/send-keys"
    });
    assert_eq!(expected, serde_json::to_value(msg).unwrap());
}

#[test]
pub fn key_sharing_no_latest_key() {
    let msg = KeySharingMessageBuilder::new()
        .id("message-id".to_string())
        .build();
    assert!(matches!(msg.unwrap_err(), MessageBuilderError::MissingKey));
}

#[test]
pub fn key_sharing_no_previous_keys() {
    let msg = KeySharingMessageBuilder::new()
        .id("message-id".to_string())
        .add_key(key1())
        .build();
    assert!(msg.is_ok());
}

fn key1() -> JsonWebKey {
    JsonWebKey {
        kty: "EC".to_string(),
        crv: "P-256".to_string(),
        x: "0000".into(),
        y: "0001".into(),
        d: "0002".into(),
        use_: "enc".into(),
        kid: "<key fingerprint>".into(),
    }
}

fn key2() -> JsonWebKey {
    JsonWebKey {
        kty: "EC".to_string(),
        crv: "P-256".to_string(),
        x: "0002".into(),
        y: "0003".into(),
        d: "0004".into(),
        use_: "enc".into(),
        kid: "<key fingerprint>".into(),
    }
}
