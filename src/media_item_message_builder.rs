use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use didcomm::{Message, MessageBuilder};
use uuid::Uuid;

use crate::{
    error::MessageBuilderError, impl_common_builder, media_item_referenced::MediaItemTrait,
    MediaItemInlined, MediaItemReferenced,
};

/// Builder for creating a DIDComm message for media items.
#[derive(Default)]
pub struct MediaItemsMessageBuilder {
    to: Option<Vec<String>>,
    from: Option<String>,
    created_time: Option<u64>,
    expires_time: Option<u64>,
    media_items_referenced: Vec<MediaItemReferenced>,
    media_items_inlined: Vec<MediaItemInlined>,
    id: Option<String>,
}

impl MediaItemsMessageBuilder {
    /// Adds a media item to include in the message.
    /// Multiple media items can be added.
    pub fn media_item_inlined(mut self, value: MediaItemInlined) -> Self {
        self.media_items_inlined.push(value);
        self
    }

    pub fn media_item_referenced(mut self, value: MediaItemReferenced) -> Self {
        self.media_items_referenced.push(value);
        self
    }

    /// Builds the DIDComm message.
    pub fn build(self) -> Result<Message, MessageBuilderError> {
        let type_ = "https://didcomm.org/media-sharing/1.0/share-media";
        let id = self.id.clone().unwrap_or(Uuid::new_v4().to_string());

        if self.media_items_inlined.is_empty() && self.media_items_referenced.is_empty() {
            return Err(MessageBuilderError::MissingMediaItem);
        }

        let value_media_items_inlined = self
            .media_items_inlined
            .iter()
            .map(|item| item.to_body_item())
            .collect::<Result<Vec<_>, MessageBuilderError>>()?;

        let value_media_items_referenced = self
            .media_items_referenced
            .iter()
            .map(|item| item.to_body_item())
            .collect::<Result<Vec<_>, MessageBuilderError>>()?;

        let body = serde_json::json!({
            "items": value_media_items_inlined
                .into_iter()
                .chain(value_media_items_referenced.into_iter())
                .collect::<Vec<_>>()
        });

        let mut didcomm_msg_builder = Message::build(id, type_.into(), body);
        didcomm_msg_builder = self.add_common_to_builder(didcomm_msg_builder)?;

        for media_item in self.media_items_inlined {
            didcomm_msg_builder = didcomm_msg_builder.attachment(media_item.into_attachment());
        }

        for media_item in self.media_items_referenced {
            didcomm_msg_builder = didcomm_msg_builder.attachment(media_item.into_attachment());
        }

        Ok(didcomm_msg_builder.finalize())
    }
}

impl_common_builder!(MediaItemsMessageBuilder);
