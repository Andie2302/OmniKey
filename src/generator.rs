use rand_core::{OsRng, RngCore};
use ssh_key::LineEnding;
use x25519_dalek::{StaticSecret, PublicKey as X25519Public};
use crate::key::Key;
use crate::key_set::KeySet;
use crate::preshared_key::PresharedKey;
use crate::private_key::PrivateKey;
use crate::public_key::PublicKey;
use crate::ssh_key_set::SSHKeySet;
use crate::wireguard_key_set::WireGuardKeySet;
pub trait KeyGenerator: Sized {
    type Error;
    fn generate() -> Result<Self, Self::Error>;
}
impl KeyGenerator for SSHKeySet {
    type Error = ssh_key::Error;
    fn generate() -> Result<Self, Self::Error> {
        let ssh_key = ssh_key::PrivateKey::random(&mut OsRng, ssh_key::Algorithm::Ed25519)?;
        let private_pem = ssh_key.to_openssh(LineEnding::LF)?;
        let public_openssh = ssh_key.public_key().to_openssh()?;
        let private_key = PrivateKey::new(Key::new(private_pem.as_bytes().to_vec()));
        let public_key = PublicKey::new(Key::new(public_openssh.as_bytes().to_vec()));
        let keyset = KeySet::new(private_key, public_key);
        Ok(SSHKeySet::new(keyset))
    }
}
pub struct WireGuardOptions {
    pub with_preshared: bool,
}
impl Default for WireGuardOptions {
    fn default() -> Self {
        Self { with_preshared: false }
    }
}
impl WireGuardOptions {
    pub fn with_preshared() -> Self {
        Self { with_preshared: true }
    }
}
fn generate_preshared_key() -> PresharedKey {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    PresharedKey::new(Key::new(bytes.to_vec()))
}
impl WireGuardKeySet {
    pub fn generate_with(opts: WireGuardOptions) -> WireGuardKeySet {
        let private_secret = StaticSecret::random_from_rng(OsRng);
        let public_key_raw = X25519Public::from(&private_secret);
        let private_key = PrivateKey::new(Key::new(private_secret.to_bytes().to_vec()));
        let public_key = PublicKey::new(Key::new(public_key_raw.to_bytes().to_vec()));
        let keyset = KeySet::new(private_key, public_key);
        let preshared_key = opts.with_preshared.then(generate_preshared_key);
        WireGuardKeySet::new(keyset, preshared_key)
    }
}
impl KeyGenerator for WireGuardKeySet {
    type Error = std::convert::Infallible;
    fn generate() -> Result<Self, Self::Error> {
        Ok(Self::generate_with(Default::default()))
    }
}