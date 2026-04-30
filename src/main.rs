use OmniKey::generate::Generate;
use OmniKey::keyset_ssh::SshKeySet;
use OmniKey::keyset_wireguard::WireguardKeySet;
use std::path::PathBuf;
use std::fs;

fn main() -> std::io::Result<()> {
    // --- Vorbereitung: Test-Ordner ---
    // Wir erstellen einen lokalen Ordner namens "test_output", um das Speichern zu prüfen.
    let test_dir = PathBuf::from("test_output");
    if !test_dir.exists() {
        fs::create_dir(&test_dir)?;
        println!("Test-Ordner 'test_output' wurde erstellt.");
    }

    // --- WireGuard ---
    println!("=== WireGuard Key-Set ===");
    let wg = WireguardKeySet::generate();
    println!("{wg}");

    // Speichern der WireGuard-Keys (als einfache Textdateien im Test-Ordner)
    fs::write(test_dir.join("wg_private.key"), wg.private_key_base64())?;
    fs::write(test_dir.join("wg_public.key"), wg.public_key_base64())?;
    println!("WireGuard-Keys in 'test_output/' gespeichert.\n");


    // --- SSH ---
    println!("=== SSH Key-Set (Ed25519) ===");
    let ssh = SshKeySet::generate();
    println!("{ssh}");

    // Hier nutzen wir die Logik zum Speichern im Test-Ordner
    let ssh_filename = "id_ed25519_test";
    let priv_path = test_dir.join(ssh_filename);
    let pub_path = test_dir.join(format!("{}.pub", ssh_filename));

    // Schreiben der Dateien
    fs::write(&priv_path, ssh.private_key_pem())?;
    fs::write(&pub_path, ssh.public_key_openssh())?;

    // Unter Linux setzen wir die Berechtigungen für den Test-Key
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&priv_path, fs::Permissions::from_mode(0o600))?;
        println!("Berechtigungen für {:?} auf 600 gesetzt.", priv_path);
    }

    println!("SSH-Keys erfolgreich in 'test_output/' gespeichert.");

    // Info: Um es später "echt" zu machen, würde man das home-crate nutzen,
    // um ~/.ssh/ zu finden, wie wir es besprochen haben.

    Ok(())
}