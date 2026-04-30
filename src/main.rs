use OmniKey::key::{Key, KeyType};
use OmniKey::key_set::KeySet;
use OmniKey::wireguard_key_set::WireGuardKeySet;

fn main() -> std::io::Result<()> {

    let public = Key::from((vec![1, 2, 3], KeyType::Public));
    let private = Key::from((vec![4, 5, 6], KeyType::Private));
    let preshared = Key::from((vec![7, 8, 9], KeyType::Preshared));
    let keyset = KeySet::new(private.clone(), public.clone());
    println!("{keyset}");
    let wireguard_key_set = WireGuardKeySet::new(keyset.clone(), Option::from(preshared.clone()));
    println!("{wireguard_key_set}");
    Ok(())
}