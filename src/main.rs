use OmniKey::key_set::KeySet;
use OmniKey::wireguard_key_set::WireGuardKeySet;
use OmniKey::public_key::PublicKey;
use OmniKey::preshared_key::PresharedKey;
use OmniKey::private_key::PrivateKey;

fn main() -> std::io::Result<()> {

    let public = PublicKey::from(vec![1, 2, 3]);
    let private = PrivateKey::from(vec![4, 5, 6]);
    let preshared = PresharedKey::from(vec![7, 8, 9]);
    let keyset = KeySet::new(private.clone(), public.clone());
    println!("{keyset}");
    let wireguard_key_set = WireGuardKeySet::new(keyset.clone(), Option::from(preshared.clone()));
    println!("{wireguard_key_set}");
    Ok(())
}