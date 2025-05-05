use didcomm_module::{MediaItem, MediaItemsMessageBuilder, MessageBuilderError};
use serde_json::json;

#[test]
pub fn media_item() {
    let media_item = MediaItem {
        id: "12345".to_string(),
        media_type: "image/png".to_string(),
        filename: "example.png".to_string(),
        description: "An example image".to_string(),
        link: "https://example.com/image.png".to_string(),
        hash: "hash-example".to_string(),
        ciphering: Some(json!({
            "alg": "aes-256-cbc",
        })),
    };

    let msg = MediaItemsMessageBuilder::new()
        .media_item(media_item.clone())
        .id("message-id".to_string())
        .build()
        .unwrap();
    let expected = json!({
        "attachments": [
            {
            "data": {
                "hash": "hash-example",
                "links": [
                "https://example.com/image.png"
                ]
            },
            "description": "An example image",
            "filename": "example.png",
            "id": "12345",
            "media_type": "image/png"
            }
        ],
        "body": {
            "items": [
            {
                "@id": msg.body["items"][0]["@id"],
                "attachment_id": "12345",
                "ciphering": {
                "alg": "aes-256-cbc"
                }
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
pub fn media_item_no_id() {
    let media_item = MediaItem {
        id: "12345".to_string(),
        media_type: "image/png".to_string(),
        filename: "example.png".to_string(),
        description: "An example image".to_string(),
        link: "https://example.com/image.png".to_string(),
        hash: "hash-example".to_string(),
        ciphering: Some(json!({
            "alg": "aes-256-cbc",
        })),
    };

    let msg = MediaItemsMessageBuilder::new()
        .media_item(media_item.clone())
        .build();
    assert!(matches!(msg.unwrap_err(), MessageBuilderError::MissingId));
}

#[test]
pub fn media_item_no_media_item() {

    let msg = MediaItemsMessageBuilder::new()
        .id("message-id".to_string())
        .build();
    assert!(matches!(msg.unwrap_err(), MessageBuilderError::MissingMediaItem));
}