# DIDComm Message Module

The DIDComm Message Module is a collection of builders to help creating key-sharing, direct and media items DIDComm messages.
Rust crate and WASM bindings are available.

These builders create a `Message` type from the [didcomm crate](https://crates.io/crates/didcomm) Further operations can be done through this type. This crate also reexposes the didcomm crate.

## `DirectMessageBuilder`

Helps to create a [Basic Message](https://didcomm.org/basicmessage/2.0/) type for user communication.

```rust
    let msg: didcomm_module::didcomm::Message = DirectMessageBuilder::new()
        .id("message-id".to_string())
        .message("Hello World.".to_string())
        .build();
```

## `MediaItemsMessageBuilder`

Helps to create a DIDComm message for [Media Sharing](https://didcomm.org/media-sharing/1.0/).

```rust
    let media_item = MediaItemReferenced {
        id: "12345".to_string(),
        media_type: "image/png".to_string(),
        filename: "example.png".to_string(),
        description: "An example image".to_string(),
        link: "https://example.com/image.png".to_string(),
        hash: "hash-example".to_string(),
        ciphering: Some(json!({
            "algorithm": "aes-256-cbc",
            "parameters" : {
                "iv": "2f3849399c60cb04b923bd33265b81c7",
                "key": "233f8ce4ac6aa125927ccd98af5750d08c9c61d98a3f5d43cbf096b4caaebe80"
            }
            })),
    };

    let msg = MediaItemsMessageBuilder::new()
        .media_item_referenced(media_item.clone())
        .id("message-id".to_string())
        .build()
        .unwrap();
```

## `KeySharingMessageBuilder`

Helps to create a DIDComm message that can be used for secure key sharing.

```rust
    let msg = KeySharingMessageBuilder::new()
        .id("message-id".to_string())
        .add_key(key1)
        .add_key(key2)
        .build()
        .unwrap();
```
