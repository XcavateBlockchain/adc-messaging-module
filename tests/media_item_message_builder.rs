use didcomm_module::{
    MediaItemInlined, MediaItemReferenced, MediaItemsMessageBuilder, MessageBuilderError,
};
use serde_json::json;

#[test]
pub fn media_item() {
    let media_item = MediaItemReferenced {
        id: "12345".to_string(),
        media_type: "image/png".to_string(),
        filename: Some("example.png".to_string()),
        description: Some("An example image".to_string()),
        link: "https://example.com/image.png".to_string(),
        hash: "hash-example".to_string(),
        ciphering: Some(json!({
            "alg": "aes-256-cbc",
        })),
    };

    let media_item_inlined = MediaItemInlined {
        id: "234".to_string(),
        media_type: "image/jpg".to_string(),
        filename: Some("example.jpg".to_string()),
        description: Some("An example jpg image".to_string()),
        base64: "base64-example".to_string(),
    };

    let msg = MediaItemsMessageBuilder::new()
        .media_item_referenced(media_item.clone())
        .media_item_inlined(media_item_inlined)
        .id("message-id".to_string())
        .build()
        .unwrap();
    let expected = json!({
        "attachments": [
            {
                "data": {
                    "base64": "base64-example",
                },
                "description": "An example jpg image",
                "filename": "example.jpg",
                "id": "234",
                "media_type": "image/jpg"
            },
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
            },
        ],
        "body": {
            "items": [
                {
                    "@id": msg.body["items"][0]["@id"],
                    "attachment_id": "234",
                },
                {
                    "@id": msg.body["items"][1]["@id"],
                    "attachment_id": "12345",
                    "ciphering": {
                        "alg": "aes-256-cbc"
                    },
                },

            ]
        },
        "id": "message-id",
        "typ": "application/didcomm-plain+json",
        "type": "https://didcomm.org/media-sharing/1.0/share-media"
    });
    assert_eq!(expected, serde_json::to_value(msg).unwrap());
}

#[test]
pub fn media_item_no_id() {
    let media_item = MediaItemReferenced {
        id: "12345".to_string(),
        media_type: "image/png".to_string(),
        filename: Some("example.png".to_string()),
        description: Some("An example image".to_string()),
        link: "https://example.com/image.png".to_string(),
        hash: "hash-example".to_string(),
        ciphering: Some(json!({
            "alg": "aes-256-cbc",
        })),
    };

    let msg = MediaItemsMessageBuilder::new()
        .media_item_referenced(media_item.clone())
        .build();
    assert_eq!(msg.unwrap().id.len(), 36);
}

#[test]
pub fn media_item_no_media_item() {
    let msg = MediaItemsMessageBuilder::new()
        .id("message-id".to_string())
        .build();
    assert!(matches!(
        msg.unwrap_err(),
        MessageBuilderError::MissingMediaItem
    ));
}
