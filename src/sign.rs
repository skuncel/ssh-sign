use std::{process::{Command, Output}};

pub struct Sign {
  file_path: String,
  key_path: String,
}

impl Sign {

  pub fn new(file_path: String, key_path: Option<String>) -> Sign {
    if let Some(provided_key_path) = key_path {
      Sign { file_path, key_path: provided_key_path }
    } else {
      let default_key_path = default_key_path();
      Sign { file_path, key_path: default_key_path }
    }
  }

  pub fn sign_file(&self) -> bool {
    let command_out = self.run_sign_command();
    self.handle_sign_output(command_out)
  }

  fn run_sign_command(&self) -> Output {
    Command::new("ssh-keygen")
      .arg("-Y").arg("sign")
      .arg("-f").arg(&self.key_path)
      .arg("-n").arg("file")
      .arg(&self.file_path)
      .output()
      .expect("Failed to run ssh-keygen command")
  }

  fn handle_sign_output(&self, output: Output) -> bool {
      if !output.stderr.is_empty() {
        let stdout = String::from_utf8(output.stdout)
          .expect("Failed to read stdout from ssh-keygen command");
        self.handle_sign_stdout(stdout);
        return true;
      } else {
        let stderr = String::from_utf8(output.stderr)
          .expect("Failed to read stderr from ssh-keygen command");
        self.handle_sign_stderr(stderr);
        return false;
      }
  }

  fn handle_sign_stdout(&self, stdout: String) {
    println!("{}", stdout);
  }

  fn handle_sign_stderr(&self, stderr: String) {
    println!("Failed signing file: {}", stderr);
  }

}

fn default_key_path() -> String {
  let home_dir = dirs::home_dir().expect("Failed to resolve home directory")
    .to_str().expect("Failed to create string from home directory path").to_string();
  format!("{}{}", home_dir, "/.ssh/file-sign.pub")
}