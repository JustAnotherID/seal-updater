use crate::cli::CliArgs;
use clap::Parser;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

mod cli;
#[path = "runner/lib.rs"]
mod lib;

#[cfg(target_family = "windows")]
const SEAL_EXE: &str = "sealdice-core.exe";
#[cfg(target_family = "unix")]
const SEAL_EXE: &str = "sealdice-core";

fn main() {
    let args = CliArgs::parse();
    println!("SealDice 升级程序 by 檀轶步棋");

    if let Err(err) = lib::run_upgrade(&args) {
        println!("\n\x1b[31m出现错误：{}\x1b[0m", err);
        return;
    }

    if args.debug {
        return;
    }

    println!("\x1b[33m海豹，启动！\x1b[0m\n");
    io::stdout().flush().unwrap();
    run_command(Path::new(&args.cwd));
}

#[cfg(target_family = "unix")]
fn run_command(path: impl AsRef<Path>) {
    use std::os::unix::process::CommandExt;
    if let Err(e) = Command::new("chmod")
        .args(["+x", &path.as_ref().join(SEAL_EXE).to_string_lossy()])
        .spawn()
    {
        println!("\x1b[31m授权失败：{}\x1b[0m\n", e);
    }
    thread::sleep(Duration::from_secs(1));
    let err = Command::new(Path::new("./").join(SEAL_EXE))
        .current_dir(path)
        .exec();
    println!("\x1b[31m启动失败：{}\x1b[0m\n", err);
}

#[cfg(target_family = "windows")]
fn run_command(path: impl AsRef<Path>) {
    thread::sleep(Duration::from_secs(2));
    if let Err(e) = Command::new("cmd")
        .current_dir(path)
        .args(["/C", "start", "", Path::new("./").join(SEAL_EXE)])
        .spawn()
    {
        println!("\x1b[31m启动失败：{}\x1b[0m\n", e);
    }
}
