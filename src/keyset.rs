use std::fmt;
use crate::key::Key;

const LABEL_PRIVATE: &str = "Private";
const LABEL_PUBLIC: &str = "Public";

/// Generisches Schlüsselpaar aus privatem und öffentlichem Schlüssel.
#[derive(Debug, Clone)]
pub struct KeySet {
    pub private_key: Key,
    pub public_key: Key,
}

impl KeySet {
    pub fn new(private_key: Key, public_key: Key) -> Self {
        Self { private_key, public_key }
    }
}

impl From<(Key, Key)> for KeySet {
    /// Constructs a `KeySet` from a `(private_key, public_key)` tuple.
    fn from((private_key, public_key): (Key, Key)) -> Self {
        Self::new(private_key, public_key)
    }
}

impl fmt::Display for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}\n{}: {}",
            LABEL_PRIVATE, self.private_key,
            LABEL_PUBLIC, self.public_key,
        )
    }
}
