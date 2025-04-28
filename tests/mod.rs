mod media_item_message_builder;
mod key_sharing_message_builder;
mod direct_message_builder;

use didcomm::did::DIDResolver;
use didcomm_module::UniversalDidResolver;


fn init() {
    let _ = env_logger::builder().is_test(true).filter_level(log::LevelFilter::Info).try_init();
}

#[tokio::test]
async fn test_did_resolver() {
    init();
    let resolver = UniversalDidResolver::new(None);
    // Test with a valid DID. Dev account.
    let did = "did:kilt:4tJv5T7iNgTKdVHWZbASFwbDvuUcRZB19asBXJnnfXz7ELbz";

    let did_doc = resolver.resolve(did).await.unwrap().unwrap();
    log::info!("DID Document: {:?}", did_doc);
}
