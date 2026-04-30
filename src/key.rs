use std::fmt;
use base64::{engine::general_purpose::STANDARD, Engine};

const BASE64_ENGINE: &'static base64::engine::general_purpose::GeneralPurpose = &STANDARD;

/// Raw key bytes with Base64 display output.
#[derive(Debug, Clone)]
pub struct Key {
    pub raw: Vec<u8>,
}

impl From<Vec<u8>> for Key {
    fn from(raw: Vec<u8>) -> Self {
        Self { raw }
    }
}

impl Key {
    /// Base64-encoded key (standard alphabet, with padding).
    pub fn as_base64(&self) -> String {
        BASE64_ENGINE.encode(&self.raw)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_base64())
    }
}
