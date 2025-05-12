use serde::{Deserialize, Serialize};
use serde_json::Value;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

/// Options for creating a DIDComm message for direct messages.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DirectMessageOptions {
    /// ID of the message.
    pub(crate) id: String,
    /// Creation time of the message.
    #[tsify(optional)]
    #[serde(rename = "createdTime")]
    pub(crate) created_time: Option<u64>,
    /// Expiration time of the message.
    #[tsify(optional)]
    #[serde(rename = "expiresTime")]
    pub(crate) expires_time: Option<u64>,
    /// Recipients of the message.
    #[tsify(optional)]
    pub(crate) to: Option<Vec<String>>,
    /// Sender of the message.
    #[tsify(optional)]
    pub(crate) from: Option<String>,
    /// Content of the message.
    pub(crate) message: String,
}

/// Options for creating a DIDComm message for key sharing.
/// Keys are structured as a [JWK set format](https://datatracker.ietf.org/doc/html/rfc7517#section-5).
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct KeySharingMessageOptions {
    /// ID of the message.
    pub(crate) id: String,
    /// Creation time of the message.
    #[tsify(optional)]
    #[serde(rename = "createdTime")]
    pub(crate) created_time: Option<u64>,
    /// Expiration time of the message.
    #[tsify(optional)]
    #[serde(rename = "expiresTime")]
    pub(crate) expires_time: Option<u64>,
    /// Recipients of the message.
    #[tsify(optional)]
    pub(crate) to: Option<Vec<String>>,
    /// Sender of the message.
    #[tsify(optional)]
    pub(crate) from: Option<String>,
    /// Json Web Keys to be shared.
    pub(crate) keys: Vec<JsonWebKey>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsonWebKey {
    /// Key type
    pub kty: String,
    /// Curve
    pub crv: String,
    /// x-coordinate
    pub x: String,
    /// y-coordinate
    pub y: String,
    /// Private key
    pub d: String,
    /// Key usage
    #[serde(rename = "use")]
    pub _use: String,
    /// Key ID
    pub kid: String,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MediaItemsMessageOptions {
    /// ID of the message.
    pub(crate) id: String,
    /// Recipients of the message.
    #[tsify(optional)]
    pub(crate) to: Option<Vec<String>>,
    /// Sender of the message.
    #[tsify(optional)]
    pub(crate) from: Option<String>,
    /// Creation time of the message.
    #[tsify(optional)]
    #[serde(rename = "createdTime")]
    pub(crate) created_time: Option<u64>,
    /// Expiration time of the message.
    #[tsify(optional)]
    #[serde(rename = "expiresTime")]
    pub(crate) expires_time: Option<u64>,
    /// List of media items.
    #[serde(rename = "mediaItems")]
    pub(crate) media_items: Vec<MediaItem>,
}

/// Media item structure.
/// Represents a media item that can be shared in a media item DIDComm message.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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
    #[tsify(type = "any | null")]
    pub ciphering: Option<Value>,
}

impl From<MediaItem> for didcomm_module::MediaItem {
    fn from(val: MediaItem) -> Self {
        didcomm_module::MediaItem {
            id: val.id,
            media_type: val.media_type,
            filename: val.filename,
            description: val.description,
            link: val.link,
            hash: val.hash,
            ciphering: val.ciphering,
        }
    }
}

impl From<JsonWebKey> for didcomm_module::JsonWebKey {
    fn from(val: JsonWebKey) -> Self {
        didcomm_module::JsonWebKey {
            kty: val.kty,
            crv: val.crv,
            x: val.x,
            y: val.y,
            d: val.d,
            _use: val._use,
            kid: val.kid,
        }
    }
}
