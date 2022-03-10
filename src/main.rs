mod cli;
mod sign;
mod verify;

use clap::Parser;
use std::process;

use crate::cli::Cli;
use crate::sign::Sign;
use crate::verify::Verify;

fn main() {
    let cli = Cli::parse();
    if cli.verify {
        perform_verify(cli)
    } else {
        perform_sign(cli);
    }
}

fn perform_sign(cli: Cli) {
    let sign_success = Sign::new(cli.file_path, cli.key_path).sign_file();
    if sign_success {
        println!("Signing file was successful!")
    } else {
        print!("Failed to sign file!")
    }
}

fn perform_verify(cli: Cli) {
    let verify_success = Verify::new(cli.identity, cli.file_path, cli.signature_path, cli.allowed_signers_path)
        .verify_file();
    if !verify_success {
        println!("Verification of file signature failed");
        process::exit(1)
    }
}