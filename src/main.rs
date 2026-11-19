mod host;
mod log;
mod remote;

use anyhow::{Context, Result, bail};
use clap::Parser;
use std::fs::{self, File};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version, about = "Deploy a directory to a UTCS Linux host and run a bash script there", long_about = None)]
struct Args {
    /// Path to the bash script to run on the remote host (piped over ssh)
    service_script: PathBuf,

    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    local_dir: PathBuf,

    #[arg(short, long)]
    remote_dir: String,

    /// Defaults to <local-dir>/logs
    #[arg(long)]
    log_dir: Option<PathBuf>,

    #[arg(short, long, default_value = "deploy")]
    prefix: String,

    /// Use this host instead of picking a random one
    #[arg(long)]
    host: Option<String>,
}

fn run(args: Args) -> Result<i32> {
    if !args.service_script.is_file() {
        bail!("service script not found: {}", args.service_script.display());
    }
    let script = fs::read(&args.service_script)
        .with_context(|| format!("reading {}", args.service_script.display()))?;
    if !args.local_dir.is_dir() {
        bail!("local dir not found: {}", args.local_dir.display());
    }

    let host = match &args.host {
        Some(h) => h.clone(),
        None => host::get_random()?,
    };
    println!("User: {}", args.user);
    println!("Host: {host}");

    let log_dir = args.log_dir.clone().unwrap_or_else(|| args.local_dir.join("logs"));
    fs::create_dir_all(&log_dir)?;
    let log_path = log::name_log(&log_dir, &args.prefix);

    remote::copy_dir(&args.user, &host, &args.local_dir, &args.remote_dir)?;

    println!("Log path: {}", log_path.display());
    let log = File::create(&log_path)?;
    remote::run_script(&args.user, &host, &script, log)
}

fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(0) => ExitCode::SUCCESS,
        Ok(code) => ExitCode::from(code.clamp(1, 255) as u8),
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}
