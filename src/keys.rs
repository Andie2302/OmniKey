//! Newtype wrappers around [`Key`] for the three roles a key can play.
//!
//! All three types are generated with the same macro so that future changes
//! (e.g. adding `serde` support) only need to happen in one place.
//!
//! `PrivateKey` and `PresharedKey` inherit `ZeroizeOnDrop` from `Key`, so
//! their bytes are automatically wiped from memory when the value is dropped.

use crate::key::Key;

macro_rules! key_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub struct $name(Key);

        impl $name {
            #[must_use]
            pub fn new(key: Key) -> Self {
                Self(key)
            }

            #[must_use]
            pub fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }

            #[must_use]
            pub fn to_base64(&self) -> String {
                self.0.to_base64()
            }
        }

        impl From<Vec<u8>> for $name {
            fn from(bytes: Vec<u8>) -> Self {
                Self::new(Key::from(bytes))
            }
        }
    };
}

key_newtype!(
    /// Holds raw private-key bytes (PEM text for SSH, raw 32 bytes for WireGuard).
    ///
    /// The bytes are zeroed on drop via `Key`'s `ZeroizeOnDrop` implementation.
    PrivateKey
);

key_newtype!(
    /// Holds raw public-key bytes.
    PublicKey
);

key_newtype!(
    /// Holds the WireGuard pre-shared key (32 random bytes).
    ///
    /// The bytes are zeroed on drop via `Key`'s `ZeroizeOnDrop` implementation.
    PresharedKey
);
