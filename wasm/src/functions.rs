use crate::types::{DirectMessageOptions, KeySharingMessageOptions, MediaItemsMessageOptions};
use didcomm_module::{DirectMessageBuilder, KeySharingMessageBuilder, MediaItemsMessageBuilder};
use wasm_bindgen::prelude::*;

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
    builder = builder.id(options.id);
    builder = builder.message(options.message);
    builder
        .build()
        .map_err(|e| format!("Failed to build message: {}", e))
        .map(|msg| serde_json::to_string(&msg).unwrap())
}

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
    builder = builder.id(options.id);

    for media_item in options.media_items {
        builder = builder.media_item(media_item.into());
    }

    builder
        .build()
        .map_err(|e| format!("Failed to build message: {}", e))
        .map(|msg| serde_json::to_string(&msg).unwrap())
}

#[wasm_bindgen(js_name = createKeySharingMessage)]
pub fn create_key_sharing_message(
    options: KeySharingMessageOptions,
) -> Result<String, String> {
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
    builder = builder.id(options.id);
    builder = builder.latest_key(options.latest_key.into());
    if let Some(previous_keys) = options.previous_keys {
        for key in previous_keys {
            builder = builder.previous_key(key.into());
        }
    }
    builder
        .build()
        .map_err(|e| format!("Failed to build message: {}", e))
        .map(|msg| serde_json::to_string(&msg).unwrap())
}