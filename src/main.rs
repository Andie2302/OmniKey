
use OmniKey::key::Key;

fn main() -> std::io::Result<()> {
    let v = vec![1,2,3];
    let s = String::from("abc");
    let key = Key::new(v, Some(s));
    println!("{key}");
    Ok(())
}