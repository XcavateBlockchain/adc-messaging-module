use didcomm_module::{Key, KeySharingMessageBuilder};
use serde_json::json;

#[test]
pub fn test() {
    let key1 = Key {
        kty: "EC".to_string(),
        crv: "P-256".to_string(),
        x: "0000".into(),
        y: "0001".into(),
        d: "0002".into(),
        usage: "enc".into(),
        kid: "<key fingerprint>".into(),
    };

    let key2 = Key {
        kty: "EC".to_string(),
        crv: "P-256".to_string(),
        x: "0002".into(),
        y: "0003".into(),
        d: "0004".into(),
        usage: "enc".into(),
        kid: "<key fingerprint>".into(),
    };

    let msg = KeySharingMessageBuilder::new()
        .id("message-id".to_string())
        .previous_key(key1.clone())
        .latest_key(key2.clone())
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
