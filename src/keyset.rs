use std::fmt;
use crate::key::Key;



/// Generisches Schlüsselpaar aus privatem und öffentlichem Schlüssel.
#[derive(Debug, Clone)]
pub struct KeySet {
    key_private: Key,
    key_public: Key,
}

impl KeySet {
    pub fn new(private_key: Key, public_key: Key) -> Self {
        Self {
            key_private: private_key,
            key_public: public_key,
        }
    }
    pub fn private_key(&self) -> &Key {
        &self.key_private
    }
    pub fn public_key(&self) -> &Key {
        &self.key_public
    }
}

impl From<(Key, Key)> for KeySet {
    /// Constructs a `KeySet` from a `(private_key, public_key)` tuple.
    fn from((private_key, public_key): (Key, Key)) -> Self {
        Self::new(private_key, public_key)
    }
}


const LABEL_PRIVATE: &str = "Private";
const LABEL_PUBLIC: &str = "Public";
impl fmt::Display for KeySet {


    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}\n{}: {}",
            LABEL_PRIVATE, self.private_key(),
            LABEL_PUBLIC, self.public_key(),
        )
    }
}
