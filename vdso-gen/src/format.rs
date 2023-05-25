use std::{
    fmt,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

pub struct Formatter {
    bin: Box<Path>,
}

impl Formatter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            bin: which::which("rustfmt")
                .expect("Cannot find rustfmt bin")
                .into_boxed_path(),
        }
    }

    pub fn format<S: fmt::Display>(&self, code: S) -> String {
        let mut child = Command::new(self.bin.as_os_str())
            .arg("--emit")
            .arg("stdout")
            .stderr(Stdio::inherit())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to format file");

        {
            let mut stdin = if let Some(stdin) = child.stdin.take() {
                stdin
            } else {
                panic!("Failed to format file");
            };
            write!(stdin, "{}", code).expect("Failed to format file");
            stdin.flush().expect("Failed to format file");
        }

        let out = child.wait_with_output().expect("Failed to format file");

        if !out.status.success() {
            panic!("Failed to format file");
        }

        String::from_utf8(out.stdout).expect("Failed to format file")
    }
}
