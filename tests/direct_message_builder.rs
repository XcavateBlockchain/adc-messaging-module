use didcomm_module::{DirectMessageBuilder, MessageBuilderError};
use serde_json::json;

#[test]
pub fn direct_message() {
    let msg: didcomm_module::didcomm::Message = DirectMessageBuilder::new()
        .id("message-id".to_string())
        .message("Hello World.".to_string())
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

#[test]
pub fn direct_message_no_message() {
    let msg = DirectMessageBuilder::new()
        .id("message-id".to_string())
        .build();

    assert!(matches!(
        msg.unwrap_err(),
        MessageBuilderError::MissingMessage
    ));
}
