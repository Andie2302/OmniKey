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

    /// Generiert ein echtes Ed25519 SSH-Schlüsselpaar im OpenSSH-Format
    pub fn generate() -> Result<Self, ssh_key::Error> {
        let ssh_key = SshPrivateKey::random(&mut OsRng, Algorithm::Ed25519)?;

        // Private Key → OpenSSH PEM-Format (z.B. für ~/.ssh/id_ed25519)
        let private_pem = ssh_key.to_openssh(LineEnding::LF)?;
        // Public Key → authorized_keys-Format (z.B. für ~/.ssh/id_ed25519.pub)
        let public_openssh = ssh_key.public_key().to_openssh()?;

        let private_key = PrivateKey::new(Key::new(private_pem.as_bytes().to_vec()));
        let public_key  = PublicKey::new(Key::new(public_openssh.as_bytes().to_vec()));

        Ok(Self {
            keyset: KeySet::new(private_key, public_key),
        })
    }

    pub fn private_key(&self) -> &PrivateKey { self.keyset.private_key() }
    pub fn public_key(&self)  -> &PublicKey  { self.keyset.public_key()  }
}

impl fmt::Display for SSHKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SSH KeySet (Ed25519):")?;

        // SSH Keys sind UTF-8 Strings, kein Base64
        let private_str = String::from_utf8_lossy(self.private_key().as_bytes());
        let public_str  = String::from_utf8_lossy(self.public_key().as_bytes());

        writeln!(f, "  Private Key (OpenSSH):")?;
        writeln!(f, "{}", private_str)?;
        writeln!(f, "  Public Key (authorized_keys):")?;
        writeln!(f, "  {}", public_str.trim())?;
        Ok(())
    }
}
