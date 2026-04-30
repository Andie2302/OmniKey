use rand_core::{OsRng, RngCore};
use crate::key::Key;
use crate::private_key::PrivateKey;
use crate::public_key::PublicKey;
use crate::ssh_key_set::SSHKeySet;
use crate::wireguard_key_set::WireGuardKeySet;
use ssh_key::LineEnding;
use x25519_dalek::{StaticSecret, PublicKey as X25519Public};
use crate::key_set::KeySet;
use crate::preshared_key::PresharedKey;



pub trait KeyGenerator {
    type Output;
    type Error;
    fn generate() -> Result<Self::Output, Self::Error>;
}


impl KeyGenerator for SSHKeySet {
    type Output = Self;
    type Error = ssh_key::Error;
    fn generate() -> Result<Self, Self::Error> {
        let ssh_key = ssh_key::PrivateKey::random(&mut rand_core::OsRng, ssh_key::Algorithm::Ed25519)?;
        let private_pem = ssh_key.to_openssh(LineEnding::LF)?;
        let public_openssh = ssh_key.public_key().to_openssh()?;
        let private_key = PrivateKey::new(Key::new(private_pem.as_bytes().to_vec()));
        let public_key  = PublicKey::new(Key::new(public_openssh.as_bytes().to_vec()));
        let keyset = KeySet::new(private_key, public_key);
        Ok(SSHKeySet::new(keyset))
    }
}

impl KeyGenerator for WireGuardKeySet {
    type Output = Self;
    type Error = std::convert::Infallible;
    fn generate() -> Result<Self, Self::Error> {
        let with_preshared = true;
        let secret = StaticSecret::random_from_rng(OsRng);
        let public = X25519Public::from(&secret);
        let private_key = PrivateKey::new(Key::new(secret.to_bytes().to_vec()));
        let public_key = PublicKey::new(Key::new(public.to_bytes().to_vec()));
        let preshared_key = if with_preshared {
            let mut bytes = [0u8; 32];
            OsRng.fill_bytes(&mut bytes);
            Some(PresharedKey::new(Key::new(bytes.to_vec())))
        } else {
            None
        };
        let keyset = KeySet::new(private_key, public_key);
        Ok(WireGuardKeySet::new(keyset, preshared_key))
    }
}