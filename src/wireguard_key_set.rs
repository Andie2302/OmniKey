use std::fmt;
use std::ops::Deref;
use crate::key::Key;
use crate::key_set::KeySet;

#[derive(Debug, Clone)]
pub struct WireGuardKeySet {
    keyset: KeySet,
    preshared_key: Option<Key>,
}

impl WireGuardKeySet {
    pub fn new(keyset: KeySet, preshared_key: Option<Key>) -> Self {
        Self { keyset, preshared_key }
    }

    pub fn preshared_key(&self) -> Option<&Key> {
        self.preshared_key.as_ref()
    }
}

impl Deref for WireGuardKeySet {
    type Target = KeySet;

    fn deref(&self) -> &Self::Target {
        &self.keyset
    }
}

impl fmt::Display for WireGuardKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "WireGuard KeySet:")?;
        write!(f, "{}", self.keyset)?;
        if let Some(key) = &self.preshared_key {
            write!(f, "  Preshared: {}", key)?;
        }
        writeln!(f,"")?;
        Ok(())
    }
}