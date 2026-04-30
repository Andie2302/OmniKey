use crate::key::Key;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PrivateKey {
    private_key: Key,
}

impl PrivateKey {
    pub fn new(private_key: Key) -> Self {
        Self { private_key }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.private_key.as_bytes()
    }

    pub fn to_base64(&self) -> String {
        self.private_key.to_base64()
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  Value: {}", self.private_key.to_base64())
    }
}

impl From<Vec<u8>> for PrivateKey {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(Key::from(bytes))
    }
}
