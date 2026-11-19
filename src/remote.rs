use anyhow::{Context, Result, bail};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Recursively copy `local_dir` to `user@host:remote_dir` with scp.
pub fn copy_dir(user: &str, host: &str, local_dir: &Path, remote_dir: &str) -> Result<()> {
    let status = Command::new("scp")
        .arg("-q")
        .arg("-r")
        .arg(local_dir)
        .arg(format!("{user}@{host}:{remote_dir}"))
        .spawn()
        .context("spawning scp")?
        .wait()?;
    if !status.success() {
        bail!("scp failed with {status}");
    }
    Ok(())
}

/// Pipe `script` into `ssh user@host`, writing its stdout to `log`.
/// Returns the remote exit code.
pub fn run_script(user: &str, host: &str, script: &[u8], log: File) -> Result<i32> {
    let mut child = Command::new("ssh")
        .arg(format!("{user}@{host}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::from(log))
        .spawn()
        .context("spawning ssh")?;
    // Dropping the handle after the write closes stdin, sending EOF.
    child.stdin.take().unwrap().write_all(script)?;
    Ok(child.wait()?.code().unwrap_or(1))
}
