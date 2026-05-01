//! Writes key material to disk with OS-appropriate file permissions.
//!
//! On Unix, private key files are created with mode **0600** (owner read/write
//! only) – the same restriction that `ssh-keygen` applies.  On other platforms
//! the default file ACLs are used; the caller is responsible for any additional
//! hardening.

use std::io::Write as _;
use std::path::{Path, PathBuf};

use crate::error::OmniKeyError;
use crate::ssh_key_set::SSHKeySet;
use crate::wireguard_key_set::WireGuardKeySet;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

pub struct KeyWriter;

impl KeyWriter {
    /// Write an SSH key pair to `dir`.
    ///
    /// Files created:
    /// - `id_ed25519`     (mode 0600 on Unix) – OpenSSH private key
    /// - `id_ed25519.pub`                      – authorized_keys public key
    pub fn write_ssh(keyset: &SSHKeySet, dir: &Path) -> Result<(), OmniKeyError> {
        validate_dir(dir)?;

        write_private(
            &dir.join("id_ed25519"),
            keyset.private_key().as_bytes(),
        )?;
        write_public(
            &dir.join("id_ed25519.pub"),
            keyset.public_key().as_bytes(),
        )?;

        eprintln!(
            "SSH keys written to:\n  {}\n  {}",
            dir.join("id_ed25519").display(),
            dir.join("id_ed25519.pub").display(),
        );
        Ok(())
    }

    /// Write a WireGuard key pair (and optional pre-shared key) to `dir`.
    ///
    /// Files created:
    /// - `wg_private.key`    (mode 0600 on Unix) – Base64 private key
    /// - `wg_public.key`                          – Base64 public key
    /// - `wg_preshared.key`  (mode 0600 on Unix) – Base64 PSK (if present)
    pub fn write_wireguard(keyset: &WireGuardKeySet, dir: &Path) -> Result<(), OmniKeyError> {
        validate_dir(dir)?;

        write_private(
            &dir.join("wg_private.key"),
            keyset.private_key().to_base64().as_bytes(),
        )?;
        write_public(
            &dir.join("wg_public.key"),
            keyset.public_key().to_base64().as_bytes(),
        )?;

        let mut written: Vec<PathBuf> =
            vec![dir.join("wg_private.key"), dir.join("wg_public.key")];

        if let Some(psk) = keyset.preshared_key() {
            let psk_path = dir.join("wg_preshared.key");
            write_private(&psk_path, psk.to_base64().as_bytes())?;
            written.push(psk_path);
        }

        eprintln!("WireGuard keys written to:");
        for p in &written {
            eprintln!("  {}", p.display());
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn validate_dir(dir: &Path) -> Result<(), OmniKeyError> {
    if !dir.is_dir() {
        return Err(OmniKeyError::OutputDir(dir.display().to_string()));
    }
    Ok(())
}

/// Write `content` to `path` with restricted permissions (0600 on Unix).
fn write_private(path: &Path, content: &[u8]) -> Result<(), OmniKeyError> {
    let mut file = open_private(path)?;
    file.write_all(content)?;
    Ok(())
}

/// Write `content` to `path` with default permissions.
fn write_public(path: &Path, content: &[u8]) -> Result<(), OmniKeyError> {
    std::fs::write(path, content)?;
    Ok(())
}

/// Open a file for writing with mode 0600 on Unix.
#[cfg(unix)]
fn open_private(path: &Path) -> Result<std::fs::File, OmniKeyError> {
    use std::os::unix::fs::OpenOptionsExt as _;
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;
    Ok(f)
}

/// On non-Unix systems, fall back to the default ACLs.
#[cfg(not(unix))]
fn open_private(path: &Path) -> Result<std::fs::File, OmniKeyError> {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    Ok(f)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{KeyGenerator, WireGuardOptions};
    use crate::ssh_key_set::SSHKeySet;
    use crate::wireguard_key_set::WireGuardKeySet;
    use std::fs;

    #[test]
    fn ssh_files_are_created() {
        let dir = std::env::temp_dir().join("omnikey_test_ssh");
        fs::create_dir_all(&dir).unwrap();

        let keyset = SSHKeySet::generate().unwrap();
        KeyWriter::write_ssh(&keyset, &dir).unwrap();

        assert!(dir.join("id_ed25519").exists());
        assert!(dir.join("id_ed25519.pub").exists());

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn wireguard_files_are_created_with_psk() {
        let dir = std::env::temp_dir().join("omnikey_test_wg");
        fs::create_dir_all(&dir).unwrap();

        let keyset = WireGuardKeySet::generate_with(WireGuardOptions::with_preshared());
        KeyWriter::write_wireguard(&keyset, &dir).unwrap();

        assert!(dir.join("wg_private.key").exists());
        assert!(dir.join("wg_public.key").exists());
        assert!(dir.join("wg_preshared.key").exists());

        fs::remove_dir_all(&dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn ssh_private_key_has_mode_600() {
        use std::os::unix::fs::PermissionsExt as _;

        let dir = std::env::temp_dir().join("omnikey_test_perm");
        fs::create_dir_all(&dir).unwrap();

        let keyset = SSHKeySet::generate().unwrap();
        KeyWriter::write_ssh(&keyset, &dir).unwrap();

        let meta = fs::metadata(dir.join("id_ed25519")).unwrap();
        let mode = meta.permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "private key must be mode 0600, got {mode:o}");

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn invalid_dir_returns_error() {
        let keyset = SSHKeySet::generate().unwrap();
        let result = KeyWriter::write_ssh(&keyset, Path::new("/nonexistent/path"));
        assert!(matches!(result, Err(OmniKeyError::OutputDir(_))));
    }
}
