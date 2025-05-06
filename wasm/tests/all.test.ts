import { expect, test } from "@jest/globals";
import { createDirectMessage, createKeySharingMessage, createMediaItemMessage } from "../node/didcomm-module-node";

test("direct message builder", () => {
    let directMessageString = createDirectMessage({
        id: "test-id-1",
        createdTime: 12345,
        expiresTime: 23456,
        to: ["did:example:test2", "did:example:test3"],
        from: "did:example:test1",
        message: "test message"
    });
    let expected = {
        id: "test-id-1",
        typ: "application/didcomm-plain+json",
        type: "https://didcomm.org/basicmessage/2.0/message",
        body: { content: "test message" },
        from: "did:example:test1",
        to: ["did:example:test2", "did:example:test3"],
        created_time: 12345,
        expires_time: 23456
    };

    expect(JSON.parse(directMessageString)).toEqual(expected);
});

test("direct message builder", () => {
    let keySharingMessageString = createKeySharingMessage({
        id: "test-id-1",
        createdTime: 1234,
        expiresTime: 2345,
        to: ["did:example:test2"],
        from: "did:example:test1",
        latestKey: {
            kty: "EC",
            crv: "P-256",
            x: "0000",
            y: "0001",
            d: "0002",
            usage: "enc",
            kid: "key-id"
        },
        previousKeys: [
            {
                kty: "EC",
                crv: "P-256",
                x: "0003",
                y: "0004",
                d: "0005",
                usage: "enc",
                kid: "key-id-2"
            }
        ]
    });
    const expected = {
        id: "test-id-1",
        typ: "application/didcomm-plain+json",
        type: "<baseuri>/share-media",
        body: {
            latestKey: { crv: "P-256", d: "0002", kid: "key-id", kty: "EC", usage: "enc", x: "0000", y: "0001" },
            previousKeys: [{ crv: "P-256", d: "0005", kid: "key-id-2", kty: "EC", usage: "enc", x: "0003", y: "0004" }]
        },
        from: "did:example:test1",
        to: ["did:example:test2"],
        created_time: 1234,
        expires_time: 2345
    };
    expect(JSON.parse(keySharingMessageString)).toEqual(expected);
});

test("direct message builder", () => {
    const mediaItemsMessageString = createMediaItemMessage({
        id: "test-id-1",
        createdTime: 1234,
        expiresTime: 2345,
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
                    alg: "AES-GCM",
                    iv: "test-iv",
                    key: "test-key"
                }
            }
        ]
    });

    const mediaItemsMessage = JSON.parse(mediaItemsMessageString);

    const expected = {
        id: "test-id-1",
        typ: "application/didcomm-plain+json",
        type: "<baseuri>/share-media",
        body: {
            items: [
                {
                    "@id": mediaItemsMessage.body.items[0]["@id"],
                    attachment_id: "media-item-id-1",
                    ciphering: { alg: "AES-GCM", iv: "test-iv", key: "test-key" }
                }
            ]
        },
        from: "did:example:test1",
        to: ["did:example:test2"],
        created_time: 1234,
        expires_time: 2345,
        attachments: [
            {
                data: { links: ["ipfs://example-link/test.pdf"], hash: "xyz.." },
                id: "media-item-id-1",
                description: "document for test",
                filename: "test.pdf",
                media_type: "pdf"
            }
        ]
    };

    expect(mediaItemsMessage).toEqual(expected);
});
