use std::fmt;
use rand_core::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};
use crate::{
    generate::Generate,
    key::Key,
    keyset::KeySet,
};

/// WireGuard-Schlüsselset: privater Schlüssel, öffentlicher Schlüssel und Pre-Shared Key.
/// Alle Schlüssel sind 32-Byte Curve25519-Schlüssel, Base64-kodiert (WireGuard-Standard).
#[derive(Debug, Clone)]
pub struct WireguardKeySet {
    pub keys: KeySet,
    pub pre_shared_key: Key,
}

fn secret_to_key(secret: StaticSecret) -> Key {
    Key::from(secret.to_bytes().to_vec())
}

impl WireguardKeySet {
    /// Gibt den privaten Schlüssel als Base64 zurück (für `wg set ... private-key`).
    pub fn private_key_base64(&self) -> String {
        self.keys.private_key().as_base64()
    }

    /// Gibt den öffentlichen Schlüssel als Base64 zurück (für `wg set ... peer ... public-key`).
    pub fn public_key_base64(&self) -> String {
        self.keys.public_key().as_base64()
    }

    /// Gibt den Pre-Shared Key als Base64 zurück (für `wg set ... peer ... preshared-key`).
    pub fn preshared_key_base64(&self) -> String {
        self.pre_shared_key.as_base64()
    }
}

impl Generate for WireguardKeySet {
    fn generate() -> Self {
        let mut rng = OsRng;

        let private_secret = StaticSecret::random_from_rng(&mut rng);
        let public_key = Key::from(PublicKey::from(&private_secret).to_bytes().to_vec());
        let private_key = secret_to_key(private_secret);

        let pre_shared_secret = StaticSecret::random_from_rng(&mut rng);
        let pre_shared_key = secret_to_key(pre_shared_secret);

        Self {
            keys: KeySet::new(private_key, public_key),
            pre_shared_key,
        }
    }
}

impl fmt::Display for WireguardKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WireGuard KeySet\n  Private:    {}\n  Public:     {}\n  PreShared:  {}",
            self.keys.private_key(),
            self.keys.public_key(),
            self.pre_shared_key,
        )
    }
}
