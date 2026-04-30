use OmniKey::key::Key;
use OmniKey::keyset::{KeySet, KeySetType};

fn main() -> std::io::Result<()> {
    // Variante 1: Mit Label (nutzt das Tupel-From)
    let key1 = Key::from((vec![1, 2, 3], String::from("abc")));
    println!("{key1}");

    // Variante 2: Ohne Label (nutzt das einfache Vec-From)
    let key2 = Key::from(vec![4, 5, 6]);
    println!("{key2}");


    let keyset = KeySet::new(key1.clone(), key2.clone(), KeySetType::Custom("Test".to_string()));
    println!("{keyset}");

    let keyset2 = KeySet::new(key1, key2, Default::default() );
    println!("{keyset2}");


    Ok(())
}