use std::fmt;
use crate::key::Key;

/// Generisches Schlüsselpaar aus privatem und öffentlichem Schlüssel.
#[derive(Debug, Clone)]
pub struct KeySet {
    pub private_key: Key,
    pub public_key:  Key,
}

impl KeySet {
    pub fn new(private_key: Key, public_key: Key) -> Self {
        Self { private_key, public_key }
    }
}

impl fmt::Display for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Private: {}\nPublic:  {}",
            self.private_key, self.public_key
        )
    }
}
