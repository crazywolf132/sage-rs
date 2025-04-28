use std::{
    ffi::OsStr,
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use crate::error::{GitError, Result};

/// Thin wrapper around `std::process::Command` with:
/// - arg quoting handled by `OsStr`
/// - working-directory support
/// - 10s default timeout (avoiding hanging forever)
pub struct GitCmd {
    cmd: Command,
    timeout: Duration,
}

impl GitCmd {
    pub fn new<I, S>(work_dir: I, args: &[S]) -> Self
    where
    I: AsRef<Path>,
    S: AsRef<OsStr>,
    {
        let mut cmd = Command::new("git");
        cmd.current_dir(work_dir)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        Self {
            cmd,
            timeout: Duration::from_secs(10),
        }
    }

    /// Override the default timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Run and return `stdout` as bytes (caller decices UTF-8 decoding)
    pub fn run(mut self) -> Result<Vec<u8>> {
        let start = Instant::now();
        let mut child = self.cmd.spawn()
        .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound {
            GitError::NotInstalled
        } else { e.into()})?;

        loop {
            if let Some(status) = child.try_wait()? {
                let out = child.wait_with_output()?;
                if !status.success() {
                    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                    let status_code = out.status.code().unwrap_or(-1);
                    return Err(GitError::Exit {
                        status: status_code,
                        stderr
                    });
                }
                return Ok(out.stdout);
            }
            if start.elapsed() > self.timeout {
                child.kill().ok();
                return Err(GitError::Timeout);
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}