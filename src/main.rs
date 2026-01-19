mod commands;
mod process;
mod proton;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "parton")]
#[command(about = "Run Windows executables in a running game's Proton context")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Ls,
    Run {
        #[arg(long, value_name = "WxH", default_missing_value = "1280x720", num_args = 0..=1, require_equals = true)]
        bypass_gamescope: Option<String>,
        appid: String,
        exe: PathBuf,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<OsString>,
    },
    Cmd {
        #[arg(long, value_name = "WxH", default_missing_value = "1280x720", num_args = 0..=1, require_equals = true)]
        bypass_gamescope: Option<String>,
        appid: String,
    },
    Exec {
        appid: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
        command: Vec<OsString>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Ls => commands::ls::run(),
        Command::Run {
            bypass_gamescope,
            appid,
            exe,
            args,
        } => commands::run::run(&appid, exe, args, bypass_gamescope),
        Command::Cmd {
            bypass_gamescope,
            appid,
        } => commands::cmd::run(&appid, bypass_gamescope),
        Command::Exec { appid, command } => commands::exec::run(&appid, command),
    }
}
