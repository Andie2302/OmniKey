use crate::key::Key;
use std::fmt;

#[derive(Debug, Clone)]
pub struct KeySet {
    private_key: Key,
    public_key: Key,
}

impl KeySet {
    pub fn new(private_key: Key, public_key: Key) -> Self {
        Self {
            private_key,
            public_key,
        }
    }

    pub fn private_key(&self) -> &Key { &self.private_key }
    pub fn public_key(&self) -> &Key { &self.public_key }
}

impl fmt::Display for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.private_key)?;
        write!(f, "{}", self.public_key)
    }
}