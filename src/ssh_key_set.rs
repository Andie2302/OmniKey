use crate::key::Key;
use crate::key_set::KeySet;

pub struct SSHKeySet {
    keyset: KeySet,
}

impl SSHKeySet {
    pub fn new(keyset: KeySet) -> Self {
        Self { keyset }
    }
    pub fn private_key(&self) -> &Key { &self.keyset.private_key() }
    pub fn public_key(&self) -> &Key { &self.keyset.public_key() }
}