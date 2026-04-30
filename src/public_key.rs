use crate::key::Key;

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub key: Key,
}

impl PublicKey {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}