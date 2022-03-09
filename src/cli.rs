use clap::Parser;

/// Smart ssh-keygen wrapper to improve the signing experience
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {

  /// Verify signature mode
  #[clap(short, long)]
  pub verify: bool,

  /// Identity to be verified against
  #[clap(short, long)]
  pub identity: Option<String>,

  /// Path to the file to process
  #[clap(short, long)]
  pub file_path: String,

  /// Path to the signature file
  #[clap(short, long)]
  pub signature_path: Option<String>,

  /// Path to the public key file
  #[clap(short, long)]
  pub key_path: Option<String>,

  /// Path to the allowed signers file
  #[clap(short, long)]
  pub allowed_signers_path: Option<String>,

}