use std::path::PathBuf;

use clap::{Parser, Subcommand};

use OmniKey::error::OmniKeyError;
use OmniKey::generator::{KeyGenerator, WireGuardOptions};
use OmniKey::ssh_key_set::SSHKeySet;
use OmniKey::wireguard_key_set::WireGuardKeySet;
use OmniKey::writer::KeyWriter;

// ---------------------------------------------------------------------------
// CLI definition
// ---------------------------------------------------------------------------

/// OmniKey – SSH and WireGuard key-pair generator
#[derive(Parser)]
#[command(name = "omnikey", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Write key files into this directory instead of printing to stdout.
    /// Private key files are created with mode 0600 on Unix.
    #[arg(short, long, global = true, value_name = "DIR")]
    output: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// Generate an Ed25519 SSH key pair
    Ssh {
        /// Embed a comment in the public key (e.g. user@host)
        #[arg(short, long, default_value = "")]
        comment: String,
    },

    /// Generate an X25519 WireGuard key pair
    Wg {
        /// Also generate a 32-byte pre-shared key
        #[arg(short, long)]
        preshared: bool,
    },

    /// Generate both SSH and WireGuard keys in one step
    All {
        /// SSH key comment (e.g. user@host)
        #[arg(short, long, default_value = "")]
        comment: String,

        /// Include a WireGuard pre-shared key
        #[arg(short, long)]
        preshared: bool,
    },
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> Result<(), OmniKeyError> {
    let cli = Cli::parse();

    match cli.command {
        Command::Ssh { comment } => {
            let ssh = SSHKeySet::generate_with_comment(&comment)?;
            match &cli.output {
                Some(dir) => KeyWriter::write_ssh(&ssh, dir)?,
                None      => println!("{ssh}"),
            }
        }

        Command::Wg { preshared } => {
            let wg = WireGuardKeySet::generate_with(WireGuardOptions { with_preshared: preshared });
            match &cli.output {
                Some(dir) => KeyWriter::write_wireguard(&wg, dir)?,
                None      => println!("{wg}"),
            }
        }

        Command::All { comment, preshared } => {
            let ssh = SSHKeySet::generate_with_comment(&comment)?;
            let wg  = WireGuardKeySet::generate_with(WireGuardOptions { with_preshared: preshared });
            match &cli.output {
                Some(dir) => {
                    KeyWriter::write_ssh(&ssh, dir)?;
                    KeyWriter::write_wireguard(&wg, dir)?;
                }
                None => {
                    println!("{ssh}");
                    println!("{wg}");
                }
            }
        }
    }

    Ok(())
}
