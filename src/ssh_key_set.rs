use std::fmt;
use rand_core::OsRng;
use ssh_key::{Algorithm, LineEnding, PrivateKey as SshPrivateKey};
use crate::key::Key;
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
    pub fn public_key(&self)  -> &PublicKey  { self.keyset.public_key()  }
}

impl fmt::Display for SSHKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SSH KeySet (Ed25519):")?;
        let private_str = String::from_utf8_lossy(self.private_key().as_bytes());
        let public_str  = String::from_utf8_lossy(self.public_key().as_bytes());
        writeln!(f, "  Private Key (OpenSSH):")?;
        writeln!(f, "{}", private_str)?;
        writeln!(f, "  Public Key (authorized_keys):")?;
        writeln!(f, "  {}", public_str.trim())?;
        Ok(())
    }
}
