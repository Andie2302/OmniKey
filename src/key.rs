use std::fmt;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;


#[derive(Debug, Clone)]
pub struct Key {
    bytes: Vec<u8>,
}

impl Key {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes}
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_base64(&self) -> String {
        STANDARD.encode(&self.bytes)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.to_base64();
        write!(f, "  Value: {value}")
    }
}

impl From<(Vec<u8>)> for Key {
    fn from((bytes): (Vec<u8>)) -> Self {
        Self::new(bytes)
    }
}

