use std::fmt;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

#[derive(Debug, Clone)]
pub struct Key {
    raw: Vec<u8>,
    label: Option<String>,
}

impl Key {
    pub fn new(raw: Vec<u8>, label: Option<String>) -> Self {
        Self { raw, label }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.raw
    }

    pub fn to_base64(&self) -> String {
        STANDARD.encode(self.as_bytes())
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key:\nLabel: {}\nValue: {}\n", self.label().unwrap_or("None"), self.to_base64())
    }
}
