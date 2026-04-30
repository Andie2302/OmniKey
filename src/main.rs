use OmniKey::wireguard_key_set::WireGuardKeySet;
use OmniKey::ssh_key_set::SSHKeySet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wg = WireGuardKeySet::generate(true);
    println!("{wg}");
    let ssh = SSHKeySet::generate()?;
    println!("{ssh}");
    Ok(())
}
