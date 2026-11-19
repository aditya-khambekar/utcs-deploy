use std::path::{Path, PathBuf};

/// `<dir>/<prefix>-<timestamp>.log`
pub fn name_log(dir: &Path, prefix: &str) -> PathBuf {
    let ts = jiff::ZonedDateTime::now().strftime("%Y-%m-%dT%H-%M-%S").to_string();
    dir.join(format!("{prefix}-{ts}.log"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_name_format() {
        let p = name_log(Path::new("/x"), "deploy");
        let s = p.to_string_lossy();
        assert!(s.starts_with("/x/deploy-") && s.ends_with(".log"));
    }
}
