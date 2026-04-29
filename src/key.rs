use std::fmt;
use base64::{engine::general_purpose::STANDARD, Engine};

/// Rohe Schlüsselbytes mit Base64-Ausgabe.
#[derive(Debug, Clone)]
pub struct Key {
    pub bytes: Vec<u8>,
}

impl Key {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Base64-kodierter Schlüssel (Standard-Alphabet, mit Padding).
    pub fn as_base64(&self) -> String {
        STANDARD.encode(&self.bytes)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_base64())
    }
}
