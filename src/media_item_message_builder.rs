use alloc::{string::String, vec::Vec};
use didcomm::{Message, MessageBuilder};

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
    media_items_referenced: Option<Vec<MediaItemReferenced>>,
    media_items_inlined: Option<Vec<MediaItemInlined>>,
    id: Option<String>,
}

impl MediaItemsMessageBuilder {
    /// Adds a media item to include in the message.
    /// Multiple media items can be added.
    pub fn media_item_inlined(mut self, value: MediaItemInlined) -> Self {
        if let Some(ref mut self_media_items) = self.media_items_inlined {
            self_media_items.push(value);
        } else {
            self.media_items_inlined = Some(vec![value]);
        }
        self
    }

    pub fn media_item_referenced(mut self, value: MediaItemReferenced) -> Self {
        if let Some(ref mut self_media_items) = self.media_items_referenced {
            self_media_items.push(value);
        } else {
            self.media_items_referenced = Some(vec![value]);
        }
        self
    }

    /// Builds the DIDComm message.
    pub fn build(self) -> Result<Message, MessageBuilderError> {
        // TODO: Replace with actual base URI? / take it from user?.
        let type_ = "<baseuri>/share-media";
        let id = self.id.clone().ok_or(MessageBuilderError::MissingId)?;

        if self.media_items_inlined.is_none() && self.media_items_referenced.is_none() {
            return Err(MessageBuilderError::MissingMediaItem);
        }

        let value_media_items_inlined = self
            .media_items_inlined
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|item| item.to_body_item())
            .collect::<Result<Vec<_>, MessageBuilderError>>()?;

        let value_media_items_referenced = self
            .media_items_referenced
            .clone()
            .unwrap_or_default()
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

        for media_item in self.media_items_inlined.unwrap_or_default() {
            didcomm_msg_builder = didcomm_msg_builder.attachment(media_item.into_attachment());
        }

        for media_item in self.media_items_referenced.unwrap_or_default() {
            didcomm_msg_builder = didcomm_msg_builder.attachment(media_item.into_attachment());
        }

        Ok(didcomm_msg_builder.finalize())
    }
}

impl_common_builder!(MediaItemsMessageBuilder);
