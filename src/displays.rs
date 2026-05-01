use std::fmt;

use crate::key_set::KeySet;
use crate::keys::{PresharedKey, PrivateKey, PublicKey};
use crate::ssh_key_set::SSHKeySet;
use crate::wireguard_key_set::WireGuardKeySet;

const SSH_INDENT: &str = "  ";

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn write_bytes(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
    write!(f, "{}", String::from_utf8_lossy(bytes))
}

/// Prints a labelled SSH block whose value spans multiple lines (PEM).
/// Trailing whitespace/newlines are trimmed before the final newline is added.
fn fmt_ssh_private_entry(f: &mut fmt::Formatter<'_>, label: &str, bytes: &[u8]) -> fmt::Result {
    writeln!(f, "{label}")?;
    write_bytes(f, bytes.trim_ascii_end())?;
    writeln!(f)
}

/// Prints a labelled SSH public key indented by `SSH_INDENT`.
fn fmt_ssh_public_entry(f: &mut fmt::Formatter<'_>, label: &str, bytes: &[u8]) -> fmt::Result {
    writeln!(f, "{label}")?;
    write!(f, "{SSH_INDENT}")?;
    write_bytes(f, bytes.trim_ascii_end())?;
    writeln!(f)
}

// ---------------------------------------------------------------------------
// Display implementations
// ---------------------------------------------------------------------------

impl fmt::Display for WireGuardKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "WireGuard KeySet:")?;
        writeln!(f, "  PrivateKey:   {}", self.private_key().to_base64())?;
        writeln!(f, "  PublicKey:    {}", self.public_key().to_base64())?;
        if let Some(psk) = self.preshared_key() {
            writeln!(f, "  PresharedKey: {}", psk.to_base64())?;
        }
        Ok(())
    }
}

impl fmt::Display for SSHKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SSH KeySet (Ed25519):")?;
        fmt_ssh_private_entry(f, "  PrivateKey (OpenSSH):", self.private_key().as_bytes())?;
        fmt_ssh_public_entry(f, "  PublicKey (authorized_keys):", self.public_key().as_bytes())
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.as_bytes()).trim_end())
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.as_bytes()).trim_end())
    }
}

impl fmt::Display for PresharedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_base64())
    }
}

impl fmt::Display for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PrivateKey: {}", self.private_key().to_base64())?;
        writeln!(f, "PublicKey:  {}", self.public_key().to_base64())
    }
}
