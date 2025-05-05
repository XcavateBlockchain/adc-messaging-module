use didcomm_module::{Key, KeySharingMessageBuilder, MessageBuilderError};
use serde_json::json;

#[test]
pub fn key_sharing() {
    let msg = KeySharingMessageBuilder::new()
        .id("message-id".to_string())
        .previous_key(key1())
        .latest_key(key2())
        .build()
        .unwrap();

    let expected = json!({
    "body": {
        "latestKey": {
        "crv": "P-256",
        "d": "0004",
        "kid": "<key fingerprint>",
        "kty": "EC",
        "usage": "enc",
        "x": "0002",
        "y": "0003"
        },
        "previousKeys": [
        {
            "crv": "P-256",
            "d": "0002",
            "kid": "<key fingerprint>",
            "kty": "EC",
            "usage": "enc",
            "x": "0000",
            "y": "0001"
        }
        ]
    },
    "id": "message-id",
    "typ": "application/didcomm-plain+json",
    "type": "<baseuri>/share-media"
    });
    assert_eq!(expected, serde_json::to_value(msg).unwrap());
}

#[test]
pub fn key_sharing_no_latest_key() {
    let msg = KeySharingMessageBuilder::new()
        .id("message-id".to_string())
        .previous_key(key1())
        .build();
    assert!(matches!(msg.unwrap_err(), MessageBuilderError::MissingLatestKey));
}

#[test]
pub fn key_sharing_no_previous_keys() {
    let msg = KeySharingMessageBuilder::new()
        .id("message-id".to_string())
        .latest_key(key1())
        .build();
    assert!(msg.is_ok());
}


fn key1() -> Key {
    Key {
        kty: "EC".to_string(),
        crv: "P-256".to_string(),
        x: "0000".into(),
        y: "0001".into(),
        d: "0002".into(),
        usage: "enc".into(),
        kid: "<key fingerprint>".into(),
    }
}

fn key2() -> Key {
    Key {
        kty: "EC".to_string(),
        crv: "P-256".to_string(),
        x: "0002".into(),
        y: "0003".into(),
        d: "0004".into(),
        usage: "enc".into(),
        kid: "<key fingerprint>".into(),
    }
}
