use OmniKey::key::Key;
use OmniKey::key_set::{KeySet};

fn main() -> std::io::Result<()> {

    let key1 = Key::from(vec![1, 2, 3]);
    println!("{key1}");

    let key2 = Key::from(vec![4, 5, 6]);
    println!("{key2}");

    let keyset = KeySet::new(key1.clone(), key2.clone());
    println!("{keyset}");

    let keyset2 = KeySet::new(key1, key2);
    println!("{keyset2}");

    Ok(())
}