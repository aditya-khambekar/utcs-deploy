## utcs-deploy
A harness to automate deploying code to UTCS Linux hosts for testing. Output is saved to a log file on your local machine for analysis. Still a work in progress.

## Architecture
- `gethost/` — Rust scraper. Fetches the UTCS unix lab status page, filters hosts that are `up`, and prints a random one (`<host>.cs.utexas.edu`).
- `service/service.sh` — your own script (gitignored; see `service.sh.example`). It is piped over SSH and executed on the remote host, so it must be self-contained.
- `remote/` — shell config and helpers:
  - `constants.sh` — derived paths (repo location, scraper location).
  - `user_constants.sh` — your settings (gitignored; see `user_constants.sh.example`).
  - `nl_helper.sh` — builds timestamped log file names.
- `deploy.sh` — entry point. Picks a host via `gethost`, runs `service.sh` on it over SSH, and writes the output to a timestamped log in `LOG_DIR`.

Note: the `scp` step that copies your code to the remote host is currently commented out in `deploy.sh`.

## Usage
Prerequisites: Rust/`cargo`, and SSH access to UTCS hosts (key-based auth recommended, since the script runs non-interactively).

1. Copy the example configs and edit them:
   ```bash
   cp remote/user_constants.sh.example remote/user_constants.sh
   cp service/service.sh.example service/service.sh
   ```
   - In `user_constants.sh`, set `USER`, `CURRENT_DIR`, `REMOTE_DIR`, `LOG_DIR`, and `LOG_PREFIX`.
   - In `service.sh`, put the commands to run on the remote host (e.g. compile and run your program).
2. Run the deployment from anywhere:
   ```bash
   bash deploy.sh
   ```
3. Find the output in `LOG_DIR` as `<LOG_PREFIX>-<timestamp>.log`.

## TODO
 - [ ] determine a better way to choose linux hosts instead of random selection
 - [ ] implement patching and diffs so that successive deploys don't copy the entire dir
 - [ ] implement multiple user configs without having to overwrite
 - [ ] change entry point from bash file to Rust CLI using `clap`.
