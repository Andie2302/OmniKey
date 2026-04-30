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
}

impl fmt::Display for PresharedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.preshared_key.to_base64();
        write!(f, "  Value: {value}")
    }
}

impl From<(Vec<u8>)> for PresharedKey {
    fn from((bytes): (Vec<u8>)) -> Self {
        Self::new(Key::from(bytes))
    }
}