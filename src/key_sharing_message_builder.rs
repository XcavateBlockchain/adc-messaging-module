use alloc::{string::String, vec::Vec};
use didcomm::Message;
use didcomm::MessageBuilder;

use crate::{error::MessageBuilderError, impl_common_builder, JsonWebKey};

/// Builder for creating a DIDComm message for key sharing.
/// Keys are structured as a [JWK set format](https://datatracker.ietf.org/doc/html/rfc7517#section-5).
#[derive(Debug, Clone, Default)]
pub struct KeySharingMessageBuilder {
    id: Option<String>,
    created_time: Option<u64>,
    expires_time: Option<u64>,
    to: Option<Vec<String>>,
    from: Option<String>,
    keys: Vec<JsonWebKey>,
}

impl KeySharingMessageBuilder {
    /// Adds a key to the key set.
    pub fn add_key(mut self, value: JsonWebKey) -> Self {
        self.keys.push(value);
        self
    }

    /// Builds the DIDComm message.
    pub fn build(self) -> Result<Message, MessageBuilderError> {
        // TODO: Replace with actual base URI? / take it from user?.
        let type_ = "<baseuri>/share-key";
        let id = self.id.clone().ok_or(MessageBuilderError::MissingId)?;
        if self.keys.is_empty() {
            return Err(MessageBuilderError::MissingKey);
        }
        let body = serde_json::json!({
            "keys" : self.keys
        });

        let mut didcomm_msg_builder = Message::build(id, type_.into(), body);

        didcomm_msg_builder = self.add_common_to_builder(didcomm_msg_builder)?;

        Ok(didcomm_msg_builder.finalize())
    }
}

impl_common_builder!(KeySharingMessageBuilder);
