use didcomm_module::DirectMessageBuilder;
use serde_json::json;

#[test]
pub fn builder() {
    let msg = DirectMessageBuilder::new()
        .message("Hello World.".to_string())
        .id("message-id".to_string())
        .build()
        .unwrap();

    let expected = json!({
        "body": {
            "content": "Hello World."
        },
        "id": "message-id",
        "typ": "application/didcomm-plain+json",
        "type": "https://didcomm.org/basicmessage/2.0/message"
    });
    assert_eq!(expected, serde_json::to_value(msg).unwrap());
}
