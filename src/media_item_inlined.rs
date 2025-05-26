use alloc::string::{String, ToString};
use didcomm::{Attachment, Base64AttachmentData};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    media_item_referenced::{MediaItemInBody, MediaItemTrait},
    MessageBuilderError,
};

// / Represents a media item with inline content in a DIDComm message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MediaItemInlined {
    /// Attachment ID.
    pub id: String,
    /// Media type of file.
    pub media_type: String,
    /// File name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// File description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// File contents encoded as base64.
    pub base64: String,
}

impl MediaItemTrait for MediaItemInlined {
    fn to_body_item(&self) -> Result<Value, MessageBuilderError> {
        serde_json::to_value(MediaItemInBody {
            id: Uuid::new_v4().to_string(),
            attachment_id: self.id.clone(),
            ciphering: None,
        })
        .map_err(|_| MessageBuilderError::SerializationError)
    }

    fn into_attachment(self) -> Attachment {
        Attachment {
            id: Some(self.id),
            media_type: Some(self.media_type),
            data: didcomm::AttachmentData::Base64 {
                value: Base64AttachmentData {
                    base64: self.base64,
                    jws: None,
                },
            },
            description: self.description,
            filename: self.filename,
            format: None,
            lastmod_time: None,
            byte_count: None,
        }
    }
}
