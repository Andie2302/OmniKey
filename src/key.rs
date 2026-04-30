use std::fmt;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

const NO_LABEL: &str = "None";

#[derive(Debug, Clone)]
pub struct Key {
    bytes: Vec<u8>,
    label: Option<String>,
}

impl Key {
    pub fn new(bytes: Vec<u8>, label: Option<String>) -> Self {
        Self { bytes, label }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_base64(&self) -> String {
        STANDARD.encode(&self.bytes)
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl From<Vec<u8>> for Key {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(bytes, None)
    }
}

impl From<(Vec<u8>, String)> for Key {
    fn from((bytes, label): (Vec<u8>, String)) -> Self {
        Self::new(bytes, Some(label))
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label().unwrap_or(NO_LABEL);
        let value = self.to_base64();
        writeln!(f, "  Label: {label}")?;
        writeln!(f, "  Value: {value}")
    }
}
