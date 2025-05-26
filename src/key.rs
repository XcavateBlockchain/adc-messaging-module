use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    pub use_: String,
    /// Key ID
    pub kid: String,
}
