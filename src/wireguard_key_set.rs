use crate::key_set::KeySet;
use crate::preshared_key::PresharedKey;
use crate::private_key::PrivateKey;
use crate::public_key::PublicKey;

#[derive(Debug, Clone)]
pub struct WireGuardKeySet {
    keyset: KeySet,
    preshared_key: Option<PresharedKey>,
}

impl WireGuardKeySet {
    pub fn new(keyset: KeySet, preshared_key: Option<PresharedKey>) -> Self {
        Self { keyset, preshared_key }
    }
    pub fn private_key(&self) -> &PrivateKey         { self.keyset.private_key() }
    pub fn public_key(&self)  -> &PublicKey          { self.keyset.public_key()  }
    pub fn preshared_key(&self) -> Option<&PresharedKey> { self.preshared_key.as_ref() }
}
