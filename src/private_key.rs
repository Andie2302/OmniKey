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
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.private_key.to_base64();
        write!(f, "  Value: {value}")
    }
}

impl From<(Vec<u8>)> for PrivateKey {
    fn from((bytes): (Vec<u8>)) -> Self {
        Self::new(Key::from(bytes))
    }
}