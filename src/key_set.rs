use std::fmt;
use crate::private_key::PrivateKey;
use crate::public_key::PublicKey;

#[derive(Debug, Clone)]
pub struct KeySet {
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl KeySet {
    pub fn new(private_key: PrivateKey, public_key: PublicKey) -> Self {
        Self { private_key, public_key }
    }

    pub fn private_key(&self) -> &PrivateKey { &self.private_key }
    pub fn public_key(&self) -> &PublicKey   { &self.public_key  }
}

impl fmt::Display for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Private Key: {}", self.private_key.to_base64())?;
        writeln!(f, "  Public Key:  {}", self.public_key.to_base64())
    }
}
