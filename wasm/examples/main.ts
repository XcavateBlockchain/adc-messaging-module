import { type IMessage, Message } from "didcomm-node";
import { createDirectMessage, createKeySharingMessage, createMediaItemMessage } from "message-module-node";

function main() {
    // This example uses the helper functions from this module to create messages.
    // Messages are created in a JSON format, which is then converted to a `Message` object from [didcomm-node](https://www.npmjs.com/package/didcomm-node) for Node.js or
    // [didcomm](https://www.npmjs.com/package/didcomm) for browser.
    // The `Message` object can be used to perform further operations. See the package documentation for more details.

    // Direct message example.
    let directMessageString = createDirectMessage({
        id: "test-id-1",
        createdTime: time(),
        expiresTime: time(1),
        to: ["did:example:test2"],
        from: "did:example:test1",
        message: "test message"
    });

    let directMsg = new Message(JSON.parse(directMessageString));
    console.log("Direct message:");
    console.log(directMsg.as_value());

    // Key sharing message example.
    let keySharingMessageString = createKeySharingMessage({
        id: "test-id-1",
        createdTime: time(),
        expiresTime: time(1),
        to: ["did:example:test2"],
        from: "did:example:test1",
        keys: [
            {
                kty: "EC",
                crv: "P-256",
                x: "0000",
                y: "0001",
                d: "0002",
                use: "enc",
                kid: "key-id"
            },

            {
                kty: "EC",
                crv: "P-256",
                x: "0003",
                y: "0004",
                d: "0005",
                use: "enc",
                kid: "key-id-2"
            }
        ]
    });

    let keySharing = new Message(JSON.parse(keySharingMessageString) as IMessage);
    console.log("Key sharing message:");
    console.log(JSON.stringify(keySharing.as_value(), null, 2));

    // Media items message example.
    let mediaItemsMessageString = createMediaItemMessage({
        id: "test-id-1",
        createdTime: time(),
        expiresTime: time(1),
        to: ["did:example:test2"],
        from: "did:example:test1",
        mediaItems: [
            {
                id: "media-item-id-1",
                media_type: "pdf",
                filename: "test.pdf",
                description: "document for test",
                link: "ipfs://example-link/test.pdf",
                hash: "xyz..",
                ciphering: {
                    algorithm: "AES-GCM",
                    parameters: {
                        iv: "test-iv",
                        key: "test-key"
                    }
                }
            }
        ]
    });

    console.log(JSON.stringify(JSON.parse(mediaItemsMessageString), null, 2));
    let mediaItems = new Message(JSON.parse(mediaItemsMessageString) as IMessage);
    console.log("Media items message:");
    console.log(JSON.stringify(mediaItems.as_value(), null, 2));
}

/**
 *  Create a time in seconds.
 *
 * @param afterYears number of years to add to the current time.
 * @returns number of seconds since epoch.
 */
function time(afterYears?: number): number {
    let date = new Date();
    if (afterYears) {
        date.setFullYear(date.getFullYear() + afterYears);
    }
    return Math.floor(date.getTime() / 1000);
}

main();
