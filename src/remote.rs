use anyhow::{Context, Result, bail};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Single-quote `path` for a POSIX shell, escaping `'` as `'\''`.
/// A leading `~/` (or a bare `~`) is left unquoted so the remote shell expands it.
fn quote_remote_path(path: &str) -> String {
    fn quote(s: &str) -> String {
        format!("'{}'", s.replace('\'', r"'\''"))
    }
    if path == "~" {
        return "~".to_string();
    }
    match path.strip_prefix("~/") {
        Some("") => "~/".to_string(),
        Some(rest) => format!("~/{}", quote(rest)),
        None => quote(path),
    }
}

/// Copy the contents of `local_dir` directly into `user@host:remote_dir`
/// (creating it if needed) by piping a local `tar` into a remote `tar` over ssh.
pub fn copy_dir(user: &str, host: &str, local_dir: &Path, remote_dir: &str) -> Result<()> {
    let mut tar = Command::new("tar")
        .arg("-C")
        .arg(local_dir)
        .args(["--no-xattrs", "--no-fflags", "--no-mac-metadata", "-cf", "-", "."])
        .env("COPYFILE_DISABLE", "1")
        .stdout(Stdio::piped())
        .spawn()
        .context("spawning tar")?;

    let dir = quote_remote_path(remote_dir);
    let ssh = Command::new("ssh")
        .arg(format!("{user}@{host}"))
        .arg(format!("mkdir -p {dir} && tar -C {dir} -xf -"))
        .stdin(Stdio::from(tar.stdout.take().unwrap()))
        .spawn();
    let mut ssh = match ssh {
        Ok(child) => child,
        Err(e) => {
            let _ = tar.kill();
            let _ = tar.wait();
            return Err(e).context("spawning ssh");
        }
    };

    let tar_status = tar.wait().context("waiting for tar")?;
    let ssh_status = ssh.wait().context("waiting for ssh")?;
    if !tar_status.success() {
        bail!("local tar failed with {tar_status}");
    }
    if !ssh_status.success() {
        bail!("remote copy (ssh mkdir/tar) failed with {ssh_status}");
    }
    Ok(())
}

/// Pipe `script` into `ssh user@host` running in `remote_dir`, writing its
/// stdout to `log`. Returns the remote exit code.
pub fn run_script(
    user: &str,
    host: &str,
    remote_dir: &str,
    script: &[u8],
    log: File,
) -> Result<i32> {
    let mut child = Command::new("ssh")
        .arg(format!("{user}@{host}"))
        .arg(format!("cd {} && bash -s", quote_remote_path(remote_dir)))
        .stdin(Stdio::piped())
        .stdout(Stdio::from(log))
        .spawn()
        .context("spawning ssh")?;
    // Dropping the handle after the write closes stdin, sending EOF.
    child.stdin.take().unwrap().write_all(script)?;
    Ok(child.wait()?.code().unwrap_or(1))
}

#[cfg(test)]
mod tests {
    use super::quote_remote_path as q;

    #[test]
    fn quoting() {
        assert_eq!(q("/tmp/a b"), "'/tmp/a b'");
        assert_eq!(q("it's"), r"'it'\''s'");
        assert_eq!(q("~/x y"), "~/'x y'");
        assert_eq!(q("~"), "~");
        assert_eq!(q("~/"), "~/");
    }
}
