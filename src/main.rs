use OmniKey::generate::Generate;
use OmniKey::keyset_ssh::SshKeySet;
use OmniKey::keyset_wireguard::WireguardKeySet;

fn main() {
    // --- WireGuard ---
    let mut wg = WireguardKeySet::generate();
    println!("=== WireGuard Key-Set ===");
    println!("{wg}");

    wg =  WireguardKeySet::generate();
    // Einzelne Felder direkt nutzbar, z.B. für wg-quick-Configs:
    println!("\n[Interface]");
    println!("PrivateKey = {}", wg.private_key_base64());
    println!("# PublicKey  = {}", wg.public_key_base64());
    println!("# PreShared  = {}", wg.preshared_key_base64());

    println!();

    // --- SSH ---
    let ssh = SshKeySet::generate();
    println!("=== SSH Key-Set (Ed25519) ===");
    println!("{ssh}");

    // Direkt als Dateien speicherbar:
    // std::fs::write("id_ed25519",     ssh.private_key_pem()).unwrap();
    // std::fs::write("id_ed25519.pub", ssh.public_key_openssh()).unwrap();
}
