use std::fmt;
use rand_core::{OsRng, RngCore};
use x25519_dalek::{StaticSecret, PublicKey as X25519Public};
use crate::key::Key;
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

    pub fn generate(with_preshared: bool) -> Self {
        let secret = StaticSecret::random_from_rng(OsRng);
        let public = X25519Public::from(&secret);
        let private_key = PrivateKey::new(Key::new(secret.to_bytes().to_vec()));
        let public_key  = PublicKey::new(Key::new(public.to_bytes().to_vec()));
        let preshared_key = if with_preshared {
            let mut bytes = [0u8; 32];
            OsRng.fill_bytes(&mut bytes);
            Some(PresharedKey::new(Key::new(bytes.to_vec())))
        } else {
            None
        };

        Self {
            keyset: KeySet::new(private_key, public_key),
            preshared_key,
        }
    }

    pub fn private_key(&self) -> &PrivateKey         { self.keyset.private_key() }
    pub fn public_key(&self)  -> &PublicKey          { self.keyset.public_key()  }
    pub fn preshared_key(&self) -> Option<&PresharedKey> { self.preshared_key.as_ref() }
}

impl fmt::Display for WireGuardKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "WireGuard KeySet:")?;
        writeln!(f, "  PrivateKey:    {}", self.private_key().to_base64())?;
        writeln!(f, "  PublicKey:     {}", self.public_key().to_base64())?;
        if let Some(psk) = &self.preshared_key {
            writeln!(f, "  PresharedKey:  {}", psk.to_base64())?;
        }
        Ok(())
    }
}
