use rand_core::{OsRng, RngCore};
use ssh_key::LineEnding;
use x25519_dalek::{PublicKey as X25519Public, StaticSecret};

use crate::error::OmniKeyError;
use crate::key::Key;
use crate::key_set::KeySet;
use crate::keys::{PresharedKey, PrivateKey, PublicKey};
use crate::ssh_key_set::SSHKeySet;
use crate::wireguard_key_set::WireGuardKeySet;

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

pub trait KeyGenerator: Sized {
    type Error;
    fn generate() -> Result<Self, Self::Error>;
}

// ---------------------------------------------------------------------------
// SSH
// ---------------------------------------------------------------------------

impl SSHKeySet {
    /// Generate an Ed25519 SSH key pair.
    ///
    /// `comment` is embedded in the public key (e.g. `user@host`).
    /// Pass an empty string to omit the comment.
    pub fn generate_with_comment(comment: &str) -> Result<Self, OmniKeyError> {
        let mut ssh_priv =
            ssh_key::PrivateKey::random(&mut OsRng, ssh_key::Algorithm::Ed25519)?;

        if !comment.is_empty() {
            ssh_priv.set_comment(comment);
        }

        let private_pem = ssh_priv.to_openssh(LineEnding::LF)?;
        let public_openssh = ssh_priv.public_key().to_openssh()?;

        let private_key = PrivateKey::new(Key::new(private_pem.as_bytes().to_vec()));
        let public_key  = PublicKey::new(Key::new(public_openssh.as_bytes().to_vec()));
        let keyset      = KeySet::new(private_key, public_key);

        Ok(SSHKeySet::new(keyset))
    }
}

impl KeyGenerator for SSHKeySet {
    type Error = OmniKeyError;

    fn generate() -> Result<Self, Self::Error> {
        Self::generate_with_comment("")
    }
}

// ---------------------------------------------------------------------------
// WireGuard
// ---------------------------------------------------------------------------

/// Options for WireGuard key generation.
#[derive(Debug, Clone, Default)]
pub struct WireGuardOptions {
    /// When `true`, a 32-byte pre-shared key is also generated.
    pub with_preshared: bool,
}

impl WireGuardOptions {
    #[must_use]
    pub fn with_preshared() -> Self {
        Self { with_preshared: true }
    }
}

fn generate_preshared_key() -> PresharedKey {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    PresharedKey::new(Key::new(bytes.to_vec()))
}

impl WireGuardKeySet {
    #[must_use]
    pub fn generate_with(opts: WireGuardOptions) -> Self {
        let secret     = StaticSecret::random_from_rng(OsRng);
        let public_raw = X25519Public::from(&secret);

        let private_key   = PrivateKey::new(Key::new(secret.to_bytes().to_vec()));
        let public_key    = PublicKey::new(Key::new(public_raw.to_bytes().to_vec()));
        let keyset        = KeySet::new(private_key, public_key);
        let preshared_key = opts.with_preshared.then(generate_preshared_key);

        WireGuardKeySet::new(keyset, preshared_key)
    }
}

impl KeyGenerator for WireGuardKeySet {
    type Error = std::convert::Infallible;

    fn generate() -> Result<Self, Self::Error> {
        Ok(Self::generate_with(WireGuardOptions::default()))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ssh_generate_produces_non_empty_keys() {
        let keyset = SSHKeySet::generate().expect("SSH generation failed");
        assert!(!keyset.private_key().as_bytes().is_empty());
        assert!(!keyset.public_key().as_bytes().is_empty());
    }

    #[test]
    fn ssh_private_key_is_pem() {
        let keyset = SSHKeySet::generate().expect("SSH generation failed");
        let pem = std::str::from_utf8(keyset.private_key().as_bytes()).expect("valid UTF-8");
        assert!(pem.contains("-----BEGIN OPENSSH PRIVATE KEY-----"));
    }

    #[test]
    fn ssh_public_key_starts_with_algo() {
        let keyset = SSHKeySet::generate().expect("SSH generation failed");
        let pub_str = std::str::from_utf8(keyset.public_key().as_bytes()).expect("valid UTF-8");
        assert!(pub_str.starts_with("ssh-ed25519"));
    }

    #[test]
    fn ssh_comment_is_embedded() {
        let keyset =
            SSHKeySet::generate_with_comment("test@host").expect("SSH generation failed");
        let pub_str = std::str::from_utf8(keyset.public_key().as_bytes()).expect("valid UTF-8");
        assert!(pub_str.contains("test@host"), "comment missing from public key");
    }

    #[test]
    fn wireguard_keys_are_32_bytes_base64() {
        let keyset = WireGuardKeySet::generate().expect("WireGuard generation failed");
        // Base64 of 32 bytes = 44 chars (with padding)
        assert_eq!(keyset.private_key().to_base64().len(), 44);
        assert_eq!(keyset.public_key().to_base64().len(), 44);
    }

    #[test]
    fn wireguard_preshared_key_present_when_requested() {
        let keyset = WireGuardKeySet::generate_with(WireGuardOptions::with_preshared());
        assert!(keyset.preshared_key().is_some());
        assert_eq!(keyset.preshared_key().unwrap().to_base64().len(), 44);
    }

    #[test]
    fn wireguard_preshared_key_absent_by_default() {
        let keyset = WireGuardKeySet::generate().expect("WireGuard generation failed");
        assert!(keyset.preshared_key().is_none());
    }

    #[test]
    fn wireguard_keys_differ_across_calls() {
        let a = WireGuardKeySet::generate().expect("first call");
        let b = WireGuardKeySet::generate().expect("second call");
        assert_ne!(a.public_key().to_base64(), b.public_key().to_base64());
    }
}
