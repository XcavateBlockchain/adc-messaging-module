use alloc::{string::String, vec::Vec};
use didcomm::{Message, MessageBuilder};

use crate::{error::MessageBuilderError, impl_common_builder, MediaItem};

/// Builder for creating a DIDComm message for media items.
#[derive(Debug, Clone, Default)]
pub struct MediaItemsMessageBuilder {
    to: Option<Vec<String>>,
    from: Option<String>,
    created_time: Option<u64>,
    expires_time: Option<u64>,
    media_items: Option<Vec<MediaItem>>,
    id: Option<String>,
}

impl MediaItemsMessageBuilder {
    /// Adds a media item to include in the message.
    /// Multiple media items can be added.
    pub fn media_item(mut self, value: MediaItem) -> Self {
        if let Some(ref mut self_media_items) = self.media_items {
            self_media_items.push(value);
        } else {
            self.media_items = Some(vec![value]);
        }
        self
    }

    /// Builds the DIDComm message.
    pub fn build(self) -> Result<Message, MessageBuilderError> {
        // TODO: Replace with actual base URI? / take it from user?.
        let type_ = "<baseuri>/share-media";
        let id = self.id.clone().ok_or(MessageBuilderError::MissingId)?;

        let media_items = self
            .media_items
            .clone()
            .ok_or(MessageBuilderError::MissingMediaItem)?;
        let value_media_items = media_items
            .iter()
            .map(|item| item.to_body_item())
            .collect::<Result<Vec<_>, MessageBuilderError>>()?;

        let body = serde_json::json!({
            "items": value_media_items
        });
        let mut didcomm_msg_builder = Message::build(id, type_.into(), body);

        for media_item in media_items {
            didcomm_msg_builder = didcomm_msg_builder.attachment(media_item.to_attachment());
        }

        didcomm_msg_builder = self.add_common_to_builder(didcomm_msg_builder)?;

        Ok(didcomm_msg_builder.finalize())
    }
}

impl_common_builder!(MediaItemsMessageBuilder);
