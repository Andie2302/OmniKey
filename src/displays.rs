use std::fmt;
use crate::key::Key;
use crate::key_set::KeySet;
use crate::preshared_key::PresharedKey;
use crate::private_key::PrivateKey;
use crate::public_key::PublicKey;
use crate::ssh_key_set::SSHKeySet;
use crate::wireguard_key_set::WireGuardKeySet;
fn write_bytes(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
    write!(f, "{}", String::from_utf8_lossy(bytes))
}
fn fmt_ssh_key_block(
    f: &mut fmt::Formatter<'_>,
    label: &str,
    bytes: &[u8],
    indent: bool,
) -> fmt::Result {
    writeln!(f, "{label}")?;
    if indent {
        write!(f, "  ")?;
    }
    write_bytes(f, bytes.trim_ascii_end())?;
    writeln!(f)
}

impl fmt::Display for WireGuardKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "WireGuard KeySet:")?;
        writeln!(f, "  PrivateKey:    {}", self.private_key().to_base64())?;
        writeln!(f, "  PublicKey:     {}", self.public_key().to_base64())?;
        if let Some(psk) = self.preshared_key() {
            writeln!(f, "  PresharedKey:  {}", psk.to_base64())?;
        }
        Ok(())
    }
}

impl fmt::Display for SSHKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SSH KeySet (Ed25519):")?;
        fmt_ssh_key_block(f, "  Private Key (OpenSSH):", self.private_key().as_bytes(), false)?;
        fmt_ssh_key_block(f, "  Public Key (authorized_keys):", self.public_key().as_bytes(), true)?;
        Ok(())
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_bytes(f, self.as_bytes())?;
        writeln!(f)
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_bytes(f, self.as_bytes())?;
        writeln!(f)
    }
}

impl fmt::Display for PresharedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_bytes(f, self.as_bytes())?;
        writeln!(f)
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