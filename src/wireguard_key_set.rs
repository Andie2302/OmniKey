use crate::key::Key;
use crate::key_set::KeySet;

pub struct WireGuardKeySet {
    keyset: KeySet,
    preshared_key: Key,
}

impl WireGuardKeySet {
    pub fn new(keyset: KeySet, preshared_key: Key) -> Self {
        Self { keyset, preshared_key }
    }
    pub fn private_key(&self) -> &Key { &self.keyset.private_key() }
    pub fn public_key(&self) -> &Key { &self.keyset.public_key() }
    pub fn preshared_key(&self) -> &Key { &self.preshared_key }
}