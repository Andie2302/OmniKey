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