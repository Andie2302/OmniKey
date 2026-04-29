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
    pub keys:           KeySet,
    pub pre_shared_key: Key,
}

impl WireguardKeySet {
    /// Gibt den privaten Schlüssel als Base64 zurück (für `wg set ... private-key`).
    pub fn private_key_base64(&self) -> String {
        self.keys.private_key.as_base64()
    }

    /// Gibt den öffentlichen Schlüssel als Base64 zurück (für `wg set ... peer ... public-key`).
    pub fn public_key_base64(&self) -> String {
        self.keys.public_key.as_base64()
    }

    /// Gibt den Pre-Shared Key als Base64 zurück (für `wg set ... peer ... preshared-key`).
    pub fn preshared_key_base64(&self) -> String {
        self.pre_shared_key.as_base64()
    }
}

impl Generate for WireguardKeySet {
    fn generate() -> Self {
        // Privater Schlüssel (Curve25519 StaticSecret)
        let private = StaticSecret::random_from_rng(OsRng);
        let public  = PublicKey::from(&private);

        // Pre-Shared Key ist ein weiterer zufälliger 32-Byte-Schlüssel
        let psk = StaticSecret::random_from_rng(OsRng);

        Self {
            keys: KeySet::new(
                Key::new(private.to_bytes().to_vec()),
                Key::new(public.to_bytes().to_vec()),
            ),
            pre_shared_key: Key::new(psk.to_bytes().to_vec()),
        }
    }
}

impl fmt::Display for WireguardKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WireGuard KeySet\n  Private:    {}\n  Public:     {}\n  PreShared:  {}",
            self.private_key_base64(),
            self.public_key_base64(),
            self.preshared_key_base64(),
        )
    }
}
