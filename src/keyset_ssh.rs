use std::fmt;

use rand_core::OsRng;
use ssh_key::{Algorithm, LineEnding, PrivateKey};
use ssh_key::private::KeypairData;

use crate::{
    generate::Generate,
    key::Key,
    keyset::KeySet,
};

/// SSH-Schlüsselset (Ed25519).
/// Enthält die rohen Schlüsselbytes sowie die fertigen OpenSSH-Strings.
#[derive(Debug, Clone)]
pub struct SshKeySet {
    /// Rohe 32-Byte-Schlüssel (privater Seed, öffentlicher Punkt).
    pub keys: KeySet,

    /// Privater Schlüssel im OpenSSH-PEM-Format (→ `~/.ssh/id_ed25519`).
    openssh_private: String,

    /// Öffentlicher Schlüssel im authorized_keys-Format (→ `~/.ssh/id_ed25519.pub`).
    openssh_public: String,

    /// Optionaler Kommentar (standardmäßig leer).
    pub comment: String,
}

impl SshKeySet {
    /// Privater Schlüssel als OpenSSH-PEM-String.
    pub fn private_key_pem(&self) -> &str {
        &self.openssh_private
    }

    /// Öffentlicher Schlüssel im `authorized_keys`-Format.
    pub fn public_key_openssh(&self) -> &str {
        &self.openssh_public
    }
}

impl Generate for SshKeySet {
    fn generate() -> Self {
        let private_key = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)
            .expect("Ed25519-Schlüsselgenerierung fehlgeschlagen");

        // OpenSSH-formatierte Strings erzeugen
        let openssh_private = private_key
            .to_openssh(LineEnding::LF)
            .expect("Fehler beim Kodieren des privaten Schlüssels")
            .to_string();

        let openssh_public = private_key
            .public_key()
            .to_openssh()
            .expect("Fehler beim Kodieren des öffentlichen Schlüssels");

        let comment = private_key.comment().to_string();

        // Rohe Ed25519-Bytes für das generische KeySet extrahieren
        let (private_bytes, public_bytes) = match private_key.key_data() {
            KeypairData::Ed25519(kp) => (
                kp.private.to_bytes().to_vec(),
                kp.public.0.to_vec(),
            ),
            _ => panic!("Unerwarteter Schlüsseltyp"),
        };

        Self {
            keys: KeySet::new(
                Key::new(private_bytes),
                Key::new(public_bytes),
            ),
            openssh_private,
            openssh_public,
            comment,
        }
    }
}

impl fmt::Display for SshKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SSH KeySet (Ed25519)\n  Public:  {}\n\n{}",
            self.openssh_public,
            self.openssh_private,
        )
    }
}
