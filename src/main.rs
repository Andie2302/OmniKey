use OmniKey::key::{Key};
use OmniKey::key_set::KeySet;
use OmniKey::wireguard_key_set::WireGuardKeySet;

fn main() -> std::io::Result<()> {

    let public = Key::from(vec![1, 2, 3]);
    let private = Key::from(vec![4, 5, 6]);
    let preshared = Key::from(vec![7, 8, 9]);
    let keyset = KeySet::new(private.clone(), public.clone());
    println!("{keyset}");
    let wireguard_key_set = WireGuardKeySet::new(keyset.clone(), Option::from(preshared.clone()));
    println!("{wireguard_key_set}");
    Ok(())
}