use std::fs;
use std::path::PathBuf;
use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    local_dir: String,

    #[arg(short, long)]
    remote_dir: String,

    #[arg(long, default_value_t = String::new())]
    log_dir: String,

    #[arg(short, long, default_value_t = String::from("deploy"))]
    prefix:String,
}

fn create_user_constants(file: &PathBuf, args: &Args) {
    let mut output = "readonly USER=\"".to_owned() + args.user.as_str() + "\"\n";
    output += &*("readonly CURRENT_DIR=\"".to_owned() + args.local_dir.as_str() + "\"\n");
    output += &*("readonly REMOTE_DIR=\"".to_owned() + args.remote_dir.as_str() + "\"\n");
    if args.log_dir.eq("") {
        output += &*("readonly LOG_DIR=\"".to_owned() + args.local_dir.as_str() + "/logs\"\n");
    } else {
        output += &*("readonly LOG_DIR=\"".to_owned() + args.log_dir.as_str() + "\"\n");
    }
    output += &*("readonly LOG_PREFIX=\"".to_owned() + args.prefix.as_str() + "\"\n");
    fs::write(file, &output).unwrap();
}

fn main() {
    let path = std::env::current_exe().unwrap();
    let project_root = path.parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap();
    let user_constants = project_root.join("remote/user_constants.sh");

    create_user_constants(&user_constants, &Args::parse());

    Command::new("bash").arg(project_root.join("deploy.sh").to_str().unwrap()).spawn().unwrap();
}