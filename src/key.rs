use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Raw byte container for any key material.
///
/// The inner bytes are **zeroed from memory** as soon as the value is
/// dropped, so sensitive key material is not left in the heap after use.
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct Key {
    bytes: Vec<u8>,
}

impl Key {
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub fn to_base64(&self) -> String {
        STANDARD.encode(&self.bytes)
    }

    /// Returns a UTF-8 string if the bytes are valid UTF-8 (e.g. PEM),
    /// otherwise falls back to Base64.
    #[must_use]
    pub fn to_utf8_or_base64(&self) -> String {
        match std::str::from_utf8(&self.bytes) {
            Ok(s) => s.to_string(),
            Err(_) => self.to_base64(),
        }
    }
}

impl From<Vec<u8>> for Key {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_base64() {
        let data = b"hello OmniKey".to_vec();
        let key = Key::new(data.clone());
        let b64 = key.to_base64();
        let decoded = STANDARD.decode(&b64).expect("valid base64");
        assert_eq!(decoded, data);
    }

    #[test]
    fn utf8_bytes_return_string() {
        let key = Key::new(b"text".to_vec());
        assert_eq!(key.to_utf8_or_base64(), "text");
    }

    #[test]
    fn non_utf8_bytes_return_base64() {
        let key = Key::new(vec![0xFF, 0xFE]);
        let result = key.to_utf8_or_base64();
        // Must be valid Base64, not raw bytes
        assert!(STANDARD.decode(&result).is_ok());
    }
}
