use crate::key::Key;

#[derive(Debug, Clone)]
pub struct PublicKey {
    public_key: Key,
}

impl PublicKey {
    pub fn new(key: Key) -> Self {
        Self { public_key: key }
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.public_key.as_bytes()
    }
    pub fn to_base64(&self) -> String {
        self.public_key.to_base64()
    }
}
impl From<Vec<u8>> for PublicKey {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(Key::from(bytes))
    }
}
