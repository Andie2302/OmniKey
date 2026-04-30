use std::fmt;
use crate::key::Key;

#[derive(Debug, Clone)]
pub struct PresharedKey {
    preshared_key: Key,
}

impl PresharedKey {
    pub fn new(preshared_key: Key) -> Self {
        Self { preshared_key }
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.preshared_key.as_bytes()
    }
    pub fn to_base64(&self) -> String {
        self.preshared_key.to_base64()
    }
}

impl fmt::Display for PresharedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  Value: {}", self.preshared_key.to_base64())
    }
}

impl From<Vec<u8>> for PresharedKey {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(Key::from(bytes))
    }
}
