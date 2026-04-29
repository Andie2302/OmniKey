/// Trait für alle Keyset-Generatoren.
/// Implementiere diesen Trait für jeden Keyset-Typ,
/// um eine einheitliche `generate()`-Schnittstelle zu erhalten.
pub trait Generate: Sized {
    fn generate() -> Self;
}
