use OmniKey::generator::{KeyGenerator, WireGuardOptions};
use OmniKey::ssh_key_set::SSHKeySet;
use OmniKey::wireguard_key_set::WireGuardKeySet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SSH: Ed25519, OpenSSH-Format
    let ssh = SSHKeySet::generate()?;
    println!("{ssh}");

    // WireGuard: ohne Preshared Key
    let wg = WireGuardKeySet::generate()?;
    println!("{wg}");

    // WireGuard: mit Preshared Key
    let wg_psk = WireGuardKeySet::generate_with(WireGuardOptions::with_preshared());
    println!("{wg_psk}");

    Ok(())
}