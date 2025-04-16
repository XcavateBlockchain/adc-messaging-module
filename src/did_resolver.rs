use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use async_trait::async_trait;
use didcomm::{
    did::{DIDDoc, DIDResolver, Service, VerificationMethod, VerificationMethodType},
    error::{Error, ErrorKind, Result as DidcommResult},
};
use json_pointer::JsonPointer;
use reqwest::{Client, Response};
use serde_json::{from_str, json, Value};
use url::Url;

const LOG_TARGET: &str = "DIDcomm::did_resolver";
type Pointer = String;
type PointerArray = [Pointer; 2];

// TODO: how can we make this generic? Is it worth it? Should be an argument in the constructor.
struct UniversalDIDDocumentPointers {
    id: JsonPointer<Pointer, PointerArray>,
    verification_method: JsonPointer<Pointer, PointerArray>,
    authentication: JsonPointer<Pointer, PointerArray>,
    key_agreement: JsonPointer<Pointer, PointerArray>,
    service: JsonPointer<Pointer, PointerArray>,
}

impl UniversalDIDDocumentPointers {
    fn new() -> Self {
        Self {
            id: Self::create_pointer("id"),
            verification_method: Self::create_pointer("verificationMethod"),
            authentication: Self::create_pointer("authentication"),
            key_agreement: Self::create_pointer("keyAgreement"),
            service: Self::create_pointer("service"),
        }
    }

    fn create_pointer(field: &str) -> JsonPointer<Pointer, PointerArray> {
        JsonPointer::new(["didDocument".to_string(), field.to_string()])
    }
}

pub struct UniversalDidResolver {
    client: Client,
    url: Url,
    pointers: UniversalDIDDocumentPointers,
}

impl UniversalDidResolver {
    pub fn new(url: Option<Url>) -> Self {
        let url = url.unwrap_or_else(|| {

            let default_url = "https://dev.uniresolver.io/1.0/identifiers";
            log::info!(target: LOG_TARGET, "No URL provided, using default UniResolver URL: {}", default_url);
            // We can save unwrap here because we know the URL is valid.
            Url::parse(default_url).unwrap()
        });
        Self {
            client: Client::new(),
            url,
            pointers: UniversalDIDDocumentPointers::new(),
        }
    }

    fn transform_verification_method(data: &Value) -> Result<Vec<VerificationMethod>, Error> {
        let verification_method_vec = data.as_array().ok_or_else(|| {
            Error::msg(ErrorKind::Malformed, "Verification method is not an array")
        })?;

        let result = verification_method_vec
            .iter()
            .filter_map(|m| {
                m.as_object().and_then(|obj| {
                    let mut obj = obj.clone();
                    if let Some(method_type) = obj.get_mut("type") {

                        let parsed_type =
                            serde_json::from_value::<VerificationMethodType>(method_type.clone())
                                .unwrap_or({
                                    log::info!(target: LOG_TARGET, "Not supported crypto detected:  {:?}", method_type);
                                    VerificationMethodType::Other
                                });

                        *method_type = json!(parsed_type);
                    }
                    serde_json::from_value(Value::Object(obj)).ok()
                })
            })
            .collect();

        Ok(result)
    }

    fn transform_service_endpoints(data: &Value) -> Result<Vec<Service>, Error> {
        let services_vec = data
            .as_array()
            .ok_or_else(|| Error::msg(ErrorKind::Malformed, "Services is not an array"))?;

        let services = services_vec
            .iter()
            .filter_map(|s| {
                serde_json::from_value::<Service>(s.clone())
                    .ok()
                    .or_else(|| {
                        // If deserialization fails, try to construct manually
                        s.as_object().and_then(|obj| {
                            let id = obj.get("id")?.as_str()?;
                            let endpoint = obj.get("serviceEndpoint")?;

                            Some(Service {
                                id: id.to_string(),
                                service_endpoint: didcomm::did::ServiceKind::Other {
                                    value: endpoint.clone(),
                                },
                            })
                        })
                    })
            })
            .collect();

        Ok(services)
    }

    /// Fetches the data from the resolver.
    async fn fetch_data(&self, did: &str) -> Result<Response, Error> {
        let response = self
            .client
            .get(&format!("{}/{}", self.url, did))
            .send()
            .await
            .map_err(|e| Error::new(ErrorKind::DIDUrlNotFound, e));

        log::info!(target: LOG_TARGET, "Received raw data from resolver: {:?}", response);

        response
    }

    /// Parses the DID document from the response data
    fn parse_did_doc(&self, response_data: String) -> Result<Option<DIDDoc>, Error> {
        let response_value =
            from_str(&response_data).map_err(|e| Error::new(ErrorKind::IoError, e))?;

        let id = self.extract_field(
            &response_value,
            &self.pointers.id,
            "Id not found".to_string(),
        )?;

        let verification_method = self.extract_and_transform(
            &response_value,
            &self.pointers.verification_method,
            "Verification method not found".to_string(),
            Self::transform_verification_method,
        )?;
        let authentication = self.extract_field_as_vec(
            &response_value,
            &self.pointers.authentication,
            "Authentication not found".to_string(),
        )?;
        let key_agreement = self.extract_field_as_vec(
            &response_value,
            &self.pointers.key_agreement,
            "Key agreement not found".to_string(),
        )?;
        let service = self.extract_and_transform(
            &response_value,
            &self.pointers.service,
            "Services not found".to_string(),
            Self::transform_service_endpoints,
        )?;

        Ok(Some(DIDDoc {
            id,
            authentication,
            key_agreement,
            verification_method,
            service,
        }))
    }

    fn extract_field(
        &self,
        response_value: &Value,
        pointer: &JsonPointer<Pointer, PointerArray>,
        error_msg: String,
    ) -> Result<String, Error> {
        pointer
            .get(response_value)
            .map_err(|_| Error::msg(ErrorKind::Malformed, error_msg))
            .and_then(|value| {
                serde_json::from_value::<String>(value.clone())
                    .map_err(|e| Error::new(ErrorKind::Malformed, e))
            })
    }

    fn extract_field_as_vec<T: serde::de::DeserializeOwned>(
        &self,
        response_value: &Value,
        pointer: &JsonPointer<Pointer, PointerArray>,
        error_msg: String,
    ) -> Result<Vec<T>, Error> {
        pointer
            .get(response_value)
            .map_err(|_| Error::msg(ErrorKind::Malformed, error_msg))
            .and_then(|value| {
                serde_json::from_value::<Vec<T>>(value.clone())
                    .map_err(|e| Error::new(ErrorKind::Malformed, e))
            })
    }

    fn extract_and_transform<T>(
        &self,
        response_value: &Value,
        pointer: &JsonPointer<Pointer, PointerArray>,
        error_msg: String,
        transform_fn: fn(&Value) -> Result<T, Error>,
    ) -> Result<T, Error> {
        pointer
            .get(response_value)
            .map_err(|_| Error::msg(ErrorKind::Malformed, error_msg))
            .and_then(transform_fn)
    }
}

#[async_trait(?Send)]
impl DIDResolver for UniversalDidResolver {
    async fn resolve(&self, did: &str) -> DidcommResult<Option<DIDDoc>> {
        let response = self.fetch_data(did).await?;
        let response_data = response
            .text()
            .await
            .map_err(|e| Error::new(ErrorKind::DIDUrlNotFound, e))?;
        self.parse_did_doc(response_data)
    }
}
