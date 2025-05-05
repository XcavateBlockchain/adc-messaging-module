use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DirectMessageOptions {
    pub(crate) id: String,
    #[tsify(optional)]
    #[serde(rename = "createdTime")]
    pub(crate) created_time: Option<u64>,
    #[tsify(optional)]
    pub(crate) expires_time: Option<u64>,
    #[tsify(optional)]
    pub(crate) to: Option<Vec<String>>,
    #[tsify(optional)]
    pub(crate) from: Option<String>,
    pub(crate) message: String,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct KeySharingMessageOptions {
    pub(crate) id: String,
    #[tsify(optional)]
    pub(crate) created_time: Option<u64>,
    #[tsify(optional)]
    pub(crate) expires_time: Option<u64>,
    #[tsify(optional)]
    pub(crate) to: Option<Vec<String>>,
    #[tsify(optional)]
    pub(crate) from: Option<String>,
    pub(crate) latest_key: Key,
    #[tsify(optional)]
    pub(crate) previous_keys: Option<Vec<Key>>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Key {
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
    pub usage: String,
    /// Key ID
    pub kid: String,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MediaItemsMessageOptions {
    pub(crate) id: String,
    #[tsify(optional)]
    pub(crate) to: Option<Vec<String>>,
    #[tsify(optional)]
    pub(crate) from: Option<String>,
    #[tsify(optional)]
    #[serde(rename = "createdTime")]
    pub(crate) created_time: Option<u64>,
    #[tsify(optional)]
    pub(crate) expires_time: Option<u64>,
    pub(crate) media_items: Vec<MediaItem>,
}

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
    pub ciphering: Option<serde_json::Value>,
}

impl Into<didcomm_module::MediaItem> for MediaItem {
    fn into(self) -> didcomm_module::MediaItem {
        didcomm_module::MediaItem {
            id: self.id,
            media_type: self.media_type,
            filename: self.filename,
            description: self.description,
            link: self.link,
            hash: self.hash,
            ciphering: self.ciphering,
        }
    }
}

impl Into<didcomm_module::Key> for Key {
    fn into(self) -> didcomm_module::Key {
        didcomm_module::Key {
            kty: self.kty,
            crv: self.crv,
            x: self.x,
            y: self.y,
            d: self.d,
            usage: self.usage,
            kid: self.kid,
        }
    }
}