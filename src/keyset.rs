use crate::key::Key;
use std::fmt;

#[derive(Debug, Clone, Default)]
pub enum KeySetType {
    SSH,
    #[default]
    WireGuard,
    Custom(String),
}

impl fmt::Display for KeySetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeySetType::SSH => write!(f, "SSH"),
            KeySetType::WireGuard => write!(f, "WireGuard"),
            KeySetType::Custom(s) => write!(f, "Custom ({})", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeySet {
    private_key: Key,
    public_key: Key,
    key_type: KeySetType,
}

impl KeySet {
    pub fn new(private_key: Key, public_key: Key, key_type: KeySetType) -> Self {
        Self {
            private_key,
            public_key,
            key_type,
        }
    }

    pub fn private_key(&self) -> &Key { &self.private_key }
    pub fn public_key(&self) -> &Key { &self.public_key }
}

impl fmt::Display for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== KeySet ({}) ===", self.key_type)?;
        write!(f, "{}", self.private_key)?;
        write!(f, "{}", self.public_key)
    }
}