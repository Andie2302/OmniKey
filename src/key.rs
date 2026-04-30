use std::fmt;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

#[derive(Debug, Clone, Default)]
pub enum KeyType {
    Public,
    Private,
    #[default]
    Preshared,
    Custom(String),
}
impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::Public => write!(f, "Public"),
            KeyType::Private => write!(f, "Private"),
            KeyType::Preshared => write!(f, "Preshared"),
            KeyType::Custom(s) => write!(f, "Custom ({})", s),
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

impl From<Vec<u8>> for Key {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(bytes, KeyType::Private)
    }
}

impl From<(Vec<u8>, KeyType)> for Key {
    fn from((bytes, k_type): (Vec<u8>, KeyType)) -> Self {
        Self::new(bytes, k_type)
    }
}

impl From<(Vec<u8>, String)> for Key {
    fn from((bytes, label): (Vec<u8>, String)) -> Self {
        Self::new(bytes, KeyType::Custom(label))
    }
}
