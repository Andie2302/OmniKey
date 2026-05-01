use crate::key_set::KeySet;
use crate::keys::{PrivateKey, PublicKey};

/// An Ed25519 SSH key pair (private key as OpenSSH PEM, public key in
/// `authorized_keys` format).
#[derive(Debug, Clone)]
pub struct SSHKeySet {
    keyset: KeySet,
}

impl SSHKeySet {
    #[must_use]
    pub fn new(keyset: KeySet) -> Self {
        Self { keyset }
    }

    #[must_use]
    pub fn private_key(&self) -> &PrivateKey {
        self.keyset.private_key()
    }

    #[must_use]
    pub fn public_key(&self) -> &PublicKey {
        self.keyset.public_key()
    }
}
