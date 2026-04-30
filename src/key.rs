use std::fmt;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

#[derive(Debug, Clone)]
pub struct Key {
    bytes: Vec<u8>,
}

impl Key {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn to_base64(&self) -> String {
        STANDARD.encode(&self.bytes)
    }
    pub fn to_utf8_or_base64(&self) -> String {
        match std::str::from_utf8(&self.bytes) {
            Ok(s) => s.to_string(),
            Err(_) => self.to_base64(),
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_base64())
    }
}

impl From<Vec<u8>> for Key {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(bytes)
    }
}
