use crate::key_set::KeySet;
use crate::private_key::PrivateKey;
use crate::public_key::PublicKey;

pub struct SSHKeySet {
    keyset: KeySet,
}

impl SSHKeySet {
    pub fn new(keyset: KeySet) -> Self {
        Self { keyset }
    }
    pub fn private_key(&self) -> &PrivateKey { self.keyset.private_key() }
    pub fn public_key(&self) -> &PublicKey { self.keyset.public_key() }
}