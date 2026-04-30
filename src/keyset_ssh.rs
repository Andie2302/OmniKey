use std::fmt;

use rand_core::OsRng;
use ssh_key::{Algorithm, LineEnding, PrivateKey};
use ssh_key::private::KeypairData;

use crate::{
    generate::Generate,
    key::Key,
    keyset::KeySet,
};

const ED25519_ALGORITHM: Algorithm = Algorithm::Ed25519;

/// Encoded OpenSSH strings derived from a private key.
struct OpenSshStrings {
    private_pem: String,
    public_openssh: String,
    comment: String,
}

/// Raw Ed25519 key bytes (private seed, public point).
struct Ed25519Bytes {
    private: Vec<u8>,
    public: Vec<u8>,
}

fn encode_openssh_strings(private_key: &PrivateKey) -> OpenSshStrings {
    let private_pem = private_key
        .to_openssh(LineEnding::LF)
        .expect("Failed to encode private key as OpenSSH PEM")
        .to_string();

    let public_openssh = private_key
        .public_key()
        .to_openssh()
        .expect("Failed to encode public key as OpenSSH");

    let comment = private_key.comment().to_string();

    OpenSshStrings { private_pem, public_openssh, comment }
}

fn extract_ed25519_bytes(private_key: &PrivateKey) -> Ed25519Bytes {
    match private_key.key_data() {
        KeypairData::Ed25519(kp) => Ed25519Bytes {
            private: kp.private.to_bytes().to_vec(),
            public: kp.public.0.to_vec(),
        },
        _ => panic!("Unexpected key type: expected Ed25519"),
    }
}

/// SSH key set (Ed25519).
/// Holds raw key bytes as well as the formatted OpenSSH strings.
#[derive(Debug, Clone)]
pub struct SshKeySet {
    /// Raw 32-byte keys (private seed, public point).
    pub keys: KeySet,
    /// Private key in OpenSSH PEM format (→ `~/.ssh/id_ed25519`).
    openssh_private: String,
    /// Public key in authorized_keys format (→ `~/.ssh/id_ed25519.pub`).
    openssh_public: String,
    /// Optional comment (empty by default).
    pub comment: String,
}

impl SshKeySet {
    /// Private key as an OpenSSH PEM string.
    pub fn private_key_pem(&self) -> &str {
        &self.openssh_private
    }

    /// Public key in `authorized_keys` format.
    pub fn public_key_openssh(&self) -> &str {
        &self.openssh_public
    }
}

impl Generate for SshKeySet {
    fn generate() -> Self {
        let private_key = PrivateKey::random(&mut OsRng, ED25519_ALGORITHM)
            .expect("Ed25519 key generation failed");

        let OpenSshStrings { private_pem, public_openssh, comment } =
            encode_openssh_strings(&private_key);

        let Ed25519Bytes { private, public } =
            extract_ed25519_bytes(&private_key);

        Self {
            keys: KeySet::new(Key::from(private), Key::from(public)),
            openssh_private: private_pem,
            openssh_public: public_openssh,
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
