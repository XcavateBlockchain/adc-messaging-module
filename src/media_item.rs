use alloc::string::{String, ToString};
use didcomm::{Attachment, LinksAttachmentData};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::error::MessageBuilderError;

// / Represents a media item in a DIDComm message.
#[derive(Debug, Clone)]
pub struct MediaItem {
    /// Attachment ID.
    pub id: String,
    /// Media type of file
    pub media_type: String,
    /// File name
    pub filename: String,
    /// File description
    pub description: String,
    /// Reference
    pub link: String,
    /// The hash of the content encoded in multi-hash format. Used as an integrity check for the attachment.
    pub hash: String,
    /// Encryption information
    pub ciphering: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct MediaItemInBody {
    #[serde(rename = "@id")]
    id: String,
    attachment_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    ciphering: Option<Value>,
}

impl MediaItem {
    pub(crate) fn to_body_item(&self) -> Result<Value, MessageBuilderError> {
        serde_json::to_value(MediaItemInBody {
            id: Uuid::new_v4().to_string(),
            attachment_id: self.id.clone(),
            ciphering: self.ciphering.clone(),
        })
        .map_err(|_| MessageBuilderError::SerializationError)
    }

    pub(crate) fn to_attachment(&self) -> Attachment {
        Attachment {
            id: Some(self.id.clone()),
            media_type: Some(self.media_type.clone()),
            data: didcomm::AttachmentData::Links {
                value: LinksAttachmentData {
                    links: vec![self.link.clone()],
                    hash: self.hash.clone(),
                    jws: None,
                },
            },
            description: Some(self.description.clone()),
            filename: Some(self.filename.clone()),
            format: None,
            lastmod_time: None,
            byte_count: None,
        }
    }
}
