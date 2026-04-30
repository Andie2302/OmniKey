use OmniKey::wireguard_key_set::WireGuardKeySet;
use OmniKey::ssh_key_set::SSHKeySet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // WireGuard: X25519 Keypair + Preshared Key
    let wg = WireGuardKeySet::generate(true);
    println!("{wg}");

    // SSH: Ed25519 Keypair im OpenSSH-Format
    let ssh = SSHKeySet::generate()?;
    println!("{ssh}");

    Ok(())
}
