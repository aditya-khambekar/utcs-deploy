## utcs-deploy
A command-line tool that copies a local directory to a UTCS Linux host, runs a bash script of your choosing there over SSH, and saves the output to a log file on your machine.

## Architecture
A single Rust binary.
- `src/main.rs` — CLI arguments and orchestration.
- `src/host.rs` — picks a random host that is `up` on the UTCS unix lab status page (`<host>.cs.utexas.edu`).
- `src/remote.rs` — copies the contents of `--local-dir` directly into `--remote-dir` (created if missing) by piping `tar` over `ssh`, then pipes your script into `ssh` to run in `--remote-dir`, writing its output to the log.
- `src/log.rs` — builds timestamped log file names.

## Usage
Prerequisites: Rust/`cargo` and SSH access to UTCS hosts.

```bash
cargo run -- <service-script> --user <utcs_username> --local-dir <path> --remote-dir <remote_path>
```

- `<service-script>` — path to the bash script to run on the remote host. It is piped over SSH, so it must be self-contained. See `service/service.sh.example`.
- `--user`, `-u` — your UTCS username.
- `--local-dir`, `-l` — local directory to copy.
- `--remote-dir`, `-r` — destination directory on the host.
- `--log-dir` — log directory (default `<local-dir>/logs`).
- `--prefix`, `-p` — log file prefix (default `deploy`).
- `--host` — use this host instead of a random one.

Output is saved to `<log-dir>/<prefix>-<timestamp>.log`.

## TODO
 - [ ] determine a better way to choose linux hosts instead of random selection
 - [ ] implement patching and diffs so that successive deploys don't copy the entire dir
