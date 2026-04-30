use std::fmt;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

#[derive(Debug, Clone)]
pub enum KeyType {
    Public,
    Private,
    Preshared
}
impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::Public => write!(f, "Public"),
            KeyType::Private => write!(f, "Private"),
            KeyType::Preshared => write!(f, "Preshared")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Key {
    bytes: Vec<u8>,
    key_type: KeyType,
}

impl Key {
    pub fn new(bytes: Vec<u8>, key_type: KeyType) -> Self {
        Self { bytes, key_type }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_base64(&self) -> String {
        STANDARD.encode(&self.bytes)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let k_type = &self.key_type;
        let value = self.to_base64();
        writeln!(f, "  Type: {k_type}")?;
        write!(f, "  Value: {value}")
    }
}

impl From<(Vec<u8>, KeyType)> for Key {
    fn from((bytes, k_type): (Vec<u8>, KeyType)) -> Self {
        Self::new(bytes, k_type)
    }
}

