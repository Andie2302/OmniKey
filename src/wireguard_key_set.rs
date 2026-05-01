use crate::key_set::KeySet;
use crate::keys::{PresharedKey, PrivateKey, PublicKey};

/// An X25519 WireGuard key pair plus an optional pre-shared key.
#[derive(Debug, Clone)]
pub struct WireGuardKeySet {
    keyset: KeySet,
    preshared_key: Option<PresharedKey>,
}

impl WireGuardKeySet {
    #[must_use]
    pub fn new(keyset: KeySet, preshared_key: Option<PresharedKey>) -> Self {
        Self { keyset, preshared_key }
    }

    #[must_use]
    pub fn private_key(&self) -> &PrivateKey {
        self.keyset.private_key()
    }

    #[must_use]
    pub fn public_key(&self) -> &PublicKey {
        self.keyset.public_key()
    }

    #[must_use]
    pub fn preshared_key(&self) -> Option<&PresharedKey> {
        self.preshared_key.as_ref()
    }
}
