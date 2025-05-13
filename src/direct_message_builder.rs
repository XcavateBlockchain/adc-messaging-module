use alloc::string::ToString;
use alloc::{string::String, vec::Vec};
use didcomm::Message;
use didcomm::MessageBuilder;
use uuid::Uuid;

use crate::error::MessageBuilderError;
use crate::impl_common_builder;

/// Builder for creating a DIDComm message for direct messages.
#[derive(Debug, Clone, Default)]
pub struct DirectMessageBuilder {
    id: Option<String>,
    created_time: Option<u64>,
    expires_time: Option<u64>,
    to: Option<Vec<String>>,
    from: Option<String>,
    message: Option<String>,
    // TODO: the didcomm crate doesn't support lang?
    // lang: Option<String>,
}

impl DirectMessageBuilder {
    /// Sets the content of the message.
    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    /// Builds the DIDComm message.
    pub fn build(self) -> Result<Message, MessageBuilderError> {
        let type_ = "https://didcomm.org/basicmessage/2.0/message";
        let id = self.id.clone().unwrap_or(Uuid::new_v4().to_string());
        let message = self
            .message
            .clone()
            .ok_or(MessageBuilderError::MissingMessage)?;

        let body = serde_json::json!({
            "content": message,
        });
        let mut didcomm_msg_builder = Message::build(id, type_.into(), body);

        didcomm_msg_builder = self.add_common_to_builder(didcomm_msg_builder)?;

        Ok(didcomm_msg_builder.finalize())
    }
}

impl_common_builder!(DirectMessageBuilder);
