use std::fs;
use std::path::PathBuf;
use std::process::Command;
use clap::Parser;
use jiff::Timestamp;
use gethost::get_random;

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

fn create_user_constants(file: &PathBuf, args: &Args, host: &str) {
    let mut output = "readonly USER=\"".to_owned() + args.user.as_str() + "\"\n";
    output += &*("readonly CURRENT_DIR=\"".to_owned() + args.local_dir.as_str() + "\"\n");
    output += &*("readonly REMOTE_DIR=\"".to_owned() + args.remote_dir.as_str() + "\"\n");
    if args.log_dir.eq("") {
        output += &*("readonly LOG_DIR=\"".to_owned() + args.local_dir.as_str() + "/logs\"\n");
    } else {
        output += &*("readonly LOG_DIR=\"".to_owned() + args.log_dir.as_str() + "\"\n");
    }
    output += &*("readonly LOG_PREFIX=\"".to_owned() + args.prefix.as_str() + "\"\n");
    output += &*("readonly HOST=\"".to_owned() + host + "\"\n");
    fs::write(file, &output).unwrap();
}

#[tokio::main]
async fn main() {
    let path = std::env::current_exe().unwrap();
    let project_root = path.parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap();

    let temporary_env = project_root.join(Timestamp::now().to_string() + "_temp");

    Command::new("mkdir").arg("-p").arg(&temporary_env).spawn().unwrap();
    Command::new("cp").arg("-a").arg(project_root.join("gethost")).arg(&temporary_env.join("gethost")).spawn().unwrap();
    Command::new("cp").arg("-r").arg(project_root.join("remote")).arg(&temporary_env.join("remote")).spawn().unwrap();
    Command::new("cp").arg("-r").arg(project_root.join("service")).arg(&temporary_env.join("service")).spawn().unwrap();
    Command::new("cp").arg(project_root.join("deploy.sh")).arg(&temporary_env.join("deploy.sh")).spawn().unwrap();

    let user_constants = temporary_env.join("remote/user_constants.sh");
        create_user_constants(&user_constants, &Args::parse(), get_random().await.as_str());

    Command::new("bash").arg(temporary_env.join("deploy.sh").to_str().unwrap()).output().unwrap();

    Command::new("rm").arg("-rf").arg(temporary_env).output().unwrap();
}