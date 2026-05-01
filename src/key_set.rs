use crate::keys::{PrivateKey, PublicKey};

/// A matched private/public key pair.
#[derive(Debug, Clone)]
pub struct KeySet {
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl KeySet {
    #[must_use]
    pub fn new(private_key: PrivateKey, public_key: PublicKey) -> Self {
        Self { private_key, public_key }
    }

    #[must_use]
    pub fn private_key(&self) -> &PrivateKey {
        &self.private_key
    }

    #[must_use]
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}
