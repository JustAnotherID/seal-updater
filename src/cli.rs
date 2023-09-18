use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CliArgs {
    /// Path to update file
    #[arg(long)]
    pub upgrade: String,
    /// Caller's PID
    #[arg(long = "pid")]
    pub pid: u32,
    /// Debug: Do not start main process
    #[arg(short)]
    pub debug: bool,
}
