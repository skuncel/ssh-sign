mod cli;
mod sign;

use clap::Parser;
use crate::cli::Cli;
use crate::sign::Sign;

fn main() {
    let cli = Cli::parse();
    if cli.verify {
        perform_verify(cli)
    } else {
        perform_sign(cli);
    }
}

fn perform_sign(cli: Cli) {
    let sign_success = Sign::new(cli.file_path, cli.key_path, cli.signature_path).sign_file();
    if sign_success {
        println!("Signing file was successful!")
    } else {
        print!("Failed to sign file!")
    }
}

fn perform_verify(cli: Cli) {
    // still to be implemented
}