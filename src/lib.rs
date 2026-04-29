pub mod generate;
pub mod key;
pub mod keyset;
pub mod keyset_ssh;
pub mod keyset_wireguard;

// Praktische Re-Exports für Nutzer des Crates
pub use generate::Generate;
pub use keyset_ssh::SshKeySet;
pub use keyset_wireguard::WireguardKeySet;
