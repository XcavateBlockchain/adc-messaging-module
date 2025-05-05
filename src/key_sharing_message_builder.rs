use alloc::{string::String, vec::Vec};
use didcomm::Message;
use didcomm::MessageBuilder;

use crate::{error::MessageBuilderError, impl_common_builder, Key};

#[derive(Debug, Clone, Default)]
pub struct KeySharingMessageBuilder {
    id: Option<String>,
    created_time: Option<u64>,
    expires_time: Option<u64>,
    to: Option<Vec<String>>,
    from: Option<String>,
    latest_key: Option<Key>,
    previous_keys: Option<Vec<Key>>,
}

impl KeySharingMessageBuilder {
    pub fn previous_key(mut self, value: Key) -> Self {
        if let Some(ref mut self_previous_keys) = self.previous_keys {
            self_previous_keys.push(value);
        } else {
            self.previous_keys = Some(vec![value]);
        }
        self
    }

    pub fn latest_key(mut self, value: Key) -> Self {
        self.latest_key = Some(value);
        self
    }

    pub fn build(self) -> Result<Message, MessageBuilderError> {
        // TODO: Replace with actual base URI? / take it from user?.
        let type_ = "<baseuri>/share-media";
        let id = self.id.clone().ok_or(MessageBuilderError::MissingId)?;
        let latest_key = self
            .latest_key
            .clone()
            .ok_or(MessageBuilderError::MissingLatestKey)?;
        let previous_keys = self.previous_keys.clone();

        let body = if let Some(previous_keys) = previous_keys {
            serde_json::json!({
                "latestKey": latest_key,
                "previousKeys" : previous_keys
            })
        } else {
            serde_json::json!({
                "latestKey": latest_key,
            })
        };

        let mut didcomm_msg_builder = Message::build(id, type_.into(), body);

        didcomm_msg_builder = self.add_common_to_builder(didcomm_msg_builder)?;

        Ok(didcomm_msg_builder.finalize())
    }
}

impl_common_builder!(KeySharingMessageBuilder);
