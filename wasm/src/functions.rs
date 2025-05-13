use crate::types::{DirectMessageOptions, KeySharingMessageOptions, MediaItemsMessageOptions};
use didcomm_module::{DirectMessageBuilder, KeySharingMessageBuilder, MediaItemsMessageBuilder};
use wasm_bindgen::prelude::*;

/// Helper function for creating a DIDComm message for direct messages.
#[wasm_bindgen(js_name = createDirectMessage)]
pub fn create_direct_message(options: DirectMessageOptions) -> Result<String, String> {
    let mut builder = DirectMessageBuilder::new();
    if let Some(from) = options.from {
        builder = builder.from(from);
    }
    if let Some(to) = options.to {
        for to in to {
            builder = builder.to(to);
        }
    }
    if let Some(created_time) = options.created_time {
        builder = builder.created_time(Some(created_time));
    }
    if let Some(expires_time) = options.expires_time {
        builder = builder.expires_time(expires_time);
    }
    if let Some(id) = options.id {
        builder = builder.id(id);
    }
    builder = builder.message(options.message);
    builder
        .build()
        .map_err(|e| format!("Failed to build message: {}", e))
        .map(|msg| serde_json::to_string(&msg).unwrap())
}

/// Helper function for creating a DIDComm message for media items.
#[wasm_bindgen (js_name = createMediaItemMessage)]
pub fn create_media_item_message(options: MediaItemsMessageOptions) -> Result<String, String> {
    let mut builder = MediaItemsMessageBuilder::new();
    if let Some(from) = options.from {
        builder = builder.from(from);
    }
    if let Some(to) = options.to {
        for to in to {
            builder = builder.to(to);
        }
    }
    if let Some(created_time) = options.created_time {
        builder = builder.created_time(Some(created_time));
    }
    if let Some(expires_time) = options.expires_time {
        builder = builder.expires_time(expires_time);
    }
    if let Some(id) = options.id {
        builder = builder.id(id);
    }

    for media_item in options.media_items {
        match media_item {
            crate::types::MediaItem::Referenced(media_item_referenced) => {
                builder = builder.media_item_referenced(media_item_referenced.into())
            }
            crate::types::MediaItem::Inlined(media_item_inlined) => {
                builder = builder.media_item_inlined(media_item_inlined.into())
            }
        }
    }

    builder
        .build()
        .map_err(|e| format!("Failed to build message: {}", e))
        .map(|msg| serde_json::to_string(&msg).unwrap())
}

/// Helper function for creating a DIDComm message for key sharing.
#[wasm_bindgen(js_name = createKeySharingMessage)]
pub fn create_key_sharing_message(options: KeySharingMessageOptions) -> Result<String, String> {
    let mut builder = KeySharingMessageBuilder::new();
    if let Some(from) = options.from {
        builder = builder.from(from);
    }
    if let Some(to) = options.to {
        for to in to {
            builder = builder.to(to);
        }
    }
    if let Some(created_time) = options.created_time {
        builder = builder.created_time(Some(created_time));
    }
    if let Some(expires_time) = options.expires_time {
        builder = builder.expires_time(expires_time);
    }
    if let Some(id) = options.id {
        builder = builder.id(id);
    }
    for key in options.keys {
        builder = builder.add_key(key.into());
    }
    builder
        .build()
        .map_err(|e| format!("Failed to build message: {}", e))
        .map(|msg| serde_json::to_string(&msg).unwrap())
}
