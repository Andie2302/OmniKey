use std::fmt;
use crate::key::Key;
use crate::preshared_key::PresharedKey;

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub public_key: Key,
}

impl PublicKey {
    pub fn new(key: Key) -> Self {
        Self { public_key: key }
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.public_key.to_base64();
        write!(f, "  Value: {value}")
    }
}

impl From<(Vec<u8>)> for PublicKey {
    fn from((bytes): (Vec<u8>)) -> Self {
        Self::new(Key::from(bytes))
    }
}