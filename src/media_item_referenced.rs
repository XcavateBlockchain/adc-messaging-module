use alloc::string::{String, ToString};
use didcomm::{Attachment, LinksAttachmentData};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::error::MessageBuilderError;

// / Represents a media item by reference in a DIDComm message.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MediaItemReferenced {
    /// Attachment ID.
    pub id: String,
    /// Media type of file
    pub media_type: String,
    /// File name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// File description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Reference
    pub link: String,
    /// The hash of the content encoded in multi-hash format. Used as an integrity check for the attachment.
    pub hash: String,
    /// Encryption information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphering: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub(crate) struct MediaItemInBody {
    #[serde(rename = "@id")]
    pub(crate) id: String,
    pub(crate) attachment_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ciphering: Option<Value>,
}

pub trait MediaItemTrait {
    fn to_body_item(&self) -> Result<Value, MessageBuilderError>;
    fn into_attachment(self) -> Attachment;
}

impl MediaItemTrait for MediaItemReferenced {
    fn to_body_item(&self) -> Result<Value, MessageBuilderError> {
        serde_json::to_value(MediaItemInBody {
            id: Uuid::new_v4().to_string(),
            attachment_id: self.id.clone(),
            ciphering: self.ciphering.clone(),
        })
        .map_err(|_| MessageBuilderError::SerializationError)
    }

    fn into_attachment(self) -> Attachment {
        Attachment {
            id: Some(self.id),
            media_type: Some(self.media_type),
            data: didcomm::AttachmentData::Links {
                value: LinksAttachmentData {
                    links: vec![self.link],
                    hash: self.hash,
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
