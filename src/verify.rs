use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    process::{Command, Output, Stdio},
};

pub struct Verify {
    identity: Option<String>,
    file_path: String,
    signature_path: String,
    allowed_signers_path: String,
}

impl Verify {
    pub fn new(
        identity: Option<String>,
        file_path: String,
        signature_path: Option<String>,
        allowed_signers_path: Option<String>,
    ) -> Verify {
        if let Some(provided_signature_path) = signature_path {
            let resolved_allowed_signers_path = resolve_allowed_signers_path(allowed_signers_path);
            Verify {
                identity,
                file_path,
                signature_path: provided_signature_path,
                allowed_signers_path: resolved_allowed_signers_path,
            }
        } else {
            let resolved_signature_path = format!("{}{}", file_path, ".sig");
            let resolved_allowed_signers_path = resolve_allowed_signers_path(allowed_signers_path);
            Verify {
                identity,
                file_path,
                signature_path: resolved_signature_path,
                allowed_signers_path: resolved_allowed_signers_path,
            }
        }
    }

    pub fn verify_file(&self) -> bool {
        if let Some(provided_identity) = &self.identity {
            let verify_output = self.run_verify_command(&provided_identity);
            self.handle_verify_output(verify_output)
        } else {
            self.check_with_all_identities()
        }
    }

    fn check_with_all_identities(&self) -> bool {
        let identities = self.collect_allowed_signers_identities();
        for identity in identities {
            let verify_output = self.run_verify_command(&identity);
            let verified = self.handle_verify_output(verify_output);
            if verified {
                return true;
            }
        }
        return false;
    }

    fn run_verify_command(&self, identity: &String) -> Output {
        Command::new("ssh-keygen")
            .arg("-Y")
            .arg("verify")
            .arg("-f")
            .arg(&self.allowed_signers_path)
            .arg("-I")
            .arg(identity)
            .arg("-n")
            .arg("file")
            .arg("-s")
            .arg(&self.signature_path)
            .stdin(Stdio::from(
                File::open(&self.file_path).expect("Couldn't open file to verify"),
            ))
            .stdout(Stdio::piped())
            .output()
            .expect("Calling ssh-keygen process failed")
    }

    fn handle_verify_output(&self, output: Output) -> bool {
        if !output.stdout.is_empty() {
            let stdout = String::from_utf8(output.stdout)
                .expect("Failed to read stdout from ssh-keygen command");
            self.handle_verify_stdout(stdout);
            return true;
        } else {
            println!(
                "{}",
                String::from_utf8(output.stderr).expect("Failed read stderr")
            );
            return false;
        }
    }

    fn handle_verify_stdout(&self, stdout: String) {
        println!("{}", stdout);
    }

    fn collect_allowed_signers_identities(&self) -> HashSet<String> {
        let allowed_signers_file =
            File::open(&self.allowed_signers_path).expect("Couldn't open allowed signers file");
        let allowed_signers_file_reader = BufReader::new(allowed_signers_file);
        let mut identities: HashSet<String> = HashSet::new();
        for line in allowed_signers_file_reader.lines() {
            let possible_identiy = self
                .get_identity_of_line(line.expect("Failed reading line in allowed signers file"));
            if possible_identiy.is_some() {
                identities.insert(possible_identiy.unwrap());
            }
        }
        return identities;
    }

    fn get_identity_of_line(&self, line: String) -> Option<String> {
        let possible_identity = line.split_whitespace().next();
        if possible_identity.is_some() {
            Some(possible_identity.unwrap().to_string())
        } else {
            None
        }
    }
}

fn resolve_allowed_signers_path(allowed_signers_path: Option<String>) -> String {
    if let Some(provided_allowed_signers_path) = allowed_signers_path {
        provided_allowed_signers_path
    } else {
        default_allowed_signers_path()
    }
}

fn default_allowed_signers_path() -> String {
    let home_dir = dirs::home_dir()
        .expect("Failed to resolve home directory")
        .to_str()
        .expect("Failed to create string from home directory path")
        .to_string();
    format!("{}{}", home_dir, "/.ssh/allowed_signers")
}
