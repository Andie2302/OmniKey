use std::fmt;
use crate::key::Key;
use crate::preshared_key::PresharedKey;

#[derive(Debug, Clone)]
pub struct PrivateKey {
    private_key: Key,
}

impl PrivateKey {
    pub fn new(private_key: Key) -> Self {
        Self { private_key }
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.private_key.to_base64();
        write!(f, "  Value: {value}")
    }
}