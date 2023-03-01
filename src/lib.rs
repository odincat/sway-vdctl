use std::process::{Command, Child};

use anyhow::{Result, bail};
use clap::Parser;

pub mod actions;
pub mod config;
pub mod state;

// CLI argument parsing
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Action {
    Create,
    Kill,
    #[clap(help = "List out active sessions")]
    List,
    Restart,
    #[clap(alias = "no", help="Manually set the next output number, in case something breaks")]
    NextOutputNumber,
    #[clap(help="Sync the next output number with 'swaymsg -t get_output'")]
    Sync,
}

#[derive(Parser, Debug, Clone)]
#[command(name = "VDctl")]
pub struct Args {
    #[arg(value_enum)]
    pub action: Action,
    #[arg(help = "Preset name to apply, alternatively a value")]
    pub value: String,
    // TODO: only required for certain commands
    // #[arg(help = "Name of preset to apply")]
    // pub preset: String,
    #[arg(long, default_value_t = false, help = "do not launch a vnc server, just create the output")]
    pub novnc: bool
}

pub fn kill_by_pid(pid: i32) -> Result<()> {
    let output = Command::new("kill")
        .arg("-TERM")
        .arg(pid.to_string())
        .output()?;

    if output.status.success() {
        //TODO: check if process under pid is still running after a short delay??
        Ok(())
    } else {
        bail!("Error killing process")
    }
}

pub fn spawn_command(command_name: &str, args: Vec<&str>) -> Result<Child, std::io::Error> {
    Command::new(command_name).args(args).spawn()
}

