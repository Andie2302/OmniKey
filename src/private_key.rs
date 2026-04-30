use crate::key::Key;

#[derive(Debug, Clone)]
pub struct PrivateKey {
    private_key: Key,
}

impl PrivateKey {
    pub fn new(private_key: Key) -> Self {
        Self { private_key }
    }
}