use thiserror::Error;

/// All errors that OmniKey can produce.
#[derive(Debug, Error)]
pub enum OmniKeyError {
    /// Wraps any error from the `ssh-key` crate.
    #[error("SSH key error: {0}")]
    Ssh(#[from] ssh_key::Error),

    /// Wraps any I/O error (e.g. writing key files to disk).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The requested output directory does not exist or is not a directory.
    #[error("Output directory not found or not a directory: {0}")]
    OutputDir(String),
}
