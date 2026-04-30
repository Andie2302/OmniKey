use std::fmt;
use crate::key::Key;
use crate::key_set::KeySet;
use crate::preshared_key::PresharedKey;
use crate::public_key::PublicKey;
use crate::ssh_key_set::SSHKeySet;
use crate::wireguard_key_set::WireGuardKeySet;

fn bytes_to_str(bytes: &[u8]) -> std::borrow::Cow<'_, str> {
    String::from_utf8_lossy(bytes)
}

impl fmt::Display for WireGuardKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "WireGuard KeySet:")?;
        writeln!(f, "  PrivateKey:    {}", self.private_key().to_base64())?;
        writeln!(f, "  PublicKey:     {}", self.public_key().to_base64())?;
        if let Some(psk) = &self.preshared_key() {
            writeln!(f, "  PresharedKey:  {}", psk.to_base64())?;
        }
        Ok(())
    }
}

impl fmt::Display for SSHKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SSH KeySet (Ed25519):")?;
        writeln!(f, "  Private Key (OpenSSH):")?;
        writeln!(f, "{}", bytes_to_str(self.private_key().as_bytes()))?;
        writeln!(f, "  Public Key (authorized_keys):")?;
        writeln!(f, "  {}", bytes_to_str(self.public_key().as_bytes()).trim())?;
        Ok(())
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", bytes_to_str(self.as_bytes()))
    }
}

impl fmt::Display for PresharedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", bytes_to_str(self.as_bytes()))
    }
}

impl fmt::Display for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Private Key: {}", self.private_key().to_base64())?;
        writeln!(f, "  Public Key:  {}", self.public_key().to_base64())
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_base64())
    }
}