use std::{
    fmt,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};

pub struct Formatter {
    bin: Box<Path>,
}

impl Formatter {
    pub fn new() -> Result<Self> {
        Ok(Self {
            bin: which::which("rustfmt")
                .wrap_err("Cannot find rustfmt bin")?
                .into_boxed_path(),
        })
    }

    pub fn format<S: fmt::Display>(&self, code: S) -> Result<String> {
        let mut child = Command::new(self.bin.as_os_str())
            .arg("--emit")
            .arg("stdout")
            .stderr(Stdio::inherit())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .wrap_err("Failed to format file")?;

        {
            let mut stdin = if let Some(stdin) = child.stdin.take() {
                stdin
            } else {
                bail!("Failed to format file");
            };
            write!(stdin, "{}", code).wrap_err("Failed to format file")?;
            stdin.flush().wrap_err("Failed to format file")?;
        }

        let out = child.wait_with_output().wrap_err("Failed to format file")?;

        if !out.status.success() {
            bail!("Failed to format file");
        }

        String::from_utf8(out.stdout).wrap_err("Failed to format file")
    }
}
