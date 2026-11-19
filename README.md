## utcs-deploy
A harness to automate deploying code to UTCS Linux hosts for testing. Output is saved to a log file on your local machine for analysis. Still a work in progress.

## Architecture
- `cli/` — Rust CLI (`clap`) and the entry point. It picks a host, copies the shell scripts into a temporary directory in the project root, generates `remote/user_constants.sh` there from your arguments, runs `deploy.sh`, then deletes the temporary directory.
- `gethost/` — Rust library crate. `get_random()` scrapes the UTCS unix lab status page, keeps hosts that are `up`, and returns a random one (`<host>.cs.utexas.edu`).
- `deploy.sh` — copies your local directory to the host with `scp`, runs `service/service.sh` there over SSH, and writes the output to a timestamped log in `LOG_DIR`.
- `service/service.sh` — the commands to run on the remote host. It is piped over SSH, so it must be self-contained. See `service.sh.example`.
- `remote/` — shell config and helpers: `constants.sh` (derived paths), `nl_helper.sh` (timestamped log names), `user_constants.sh.example` (the settings the CLI generates for you).

## Usage
Prerequisites: Rust/`cargo`, and SSH access to UTCS hosts (key-based auth recommended, since the scripts run non-interactively).

1. Edit `service/service.sh` to compile and run your code on the remote host.
2. Run the CLI from the `cli/` directory:
   ```bash
   cargo run -- --user <utcs_username> --local-dir /abs/path/to/repo --remote-dir /u/<utcs_username>/path/to/remote
   ```
   Optional flags: `--log-dir` (default `<local-dir>/logs`) and `--prefix` (log file prefix, default `deploy`).
3. Find the output in the log directory as `<prefix>-<timestamp>.log`.

Note: the CLI locates the project root relative to its own binary (`cli/target/debug/cli`), so run it with `cargo run` or the built debug binary.

## TODO
 - [ ] add an argument for a path to the service file
 - [ ] determine a better way to choose linux hosts instead of random selection
 - [ ] implement patching and diffs so that successive deploys don't copy the entire dir
 - [ ] implement multiple user configs without having to overwrite
 - [ ] finish moving the entry point from the bash file to the Rust CLI (`deploy.sh` is still invoked by it)
