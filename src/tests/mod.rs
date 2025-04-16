use didcomm::did::DIDResolver;

use crate::did_resolver::UniversalDidResolver;

#[tokio::test]
async fn test_did_resolver() {
    let resolver = UniversalDidResolver::new(None);
    // Test with a valid DID. Dev account.
    let did = "did:kilt:4tJv5T7iNgTKdVHWZbASFwbDvuUcRZB19asBXJnnfXz7ELbz";
    let _did_doc = resolver.resolve(did).await.unwrap().unwrap();
}
