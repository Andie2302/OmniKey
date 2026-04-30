use OmniKey::key::Key;

fn main() -> std::io::Result<()> {
    // Variante 1: Mit Label (nutzt das Tupel-From)
    let key1 = Key::from((vec![1, 2, 3], String::from("abc")));
    println!("{key1}");

    // Variante 2: Ohne Label (nutzt das einfache Vec-From)
    let key2 = Key::from(vec![4, 5, 6]);
    println!("{key2}");

    Ok(())
}