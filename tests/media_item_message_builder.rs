use didcomm_module::{MediaItem, MediaItemsMessageBuilder};
use serde_json::json;

#[test]
pub fn builder() {
    let media_item = MediaItem {
        id: "12345".to_string(),
        media_type: "image/png".to_string(),
        filename: "example.png".to_string(),
        description: "An example image".to_string(),
        link: "https://example.com/image.png".to_string(),
        hash: "hash-example".to_string(),
        ciphering: None,
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
                "attachment_id": "12345"
            }
            ]
        },
        "id": "message-id",
        "typ": "application/didcomm-plain+json",
        "type": "<baseuri>/share-media"
    });
    assert_eq!(expected, serde_json::to_value(msg).unwrap());
}
