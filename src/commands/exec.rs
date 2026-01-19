use crate::proton::WineserverInfo;
use anyhow::{bail, Result};
use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub fn run(appid: &str, command: Vec<OsString>) -> Result<()> {
    let info = WineserverInfo::find_by_appid(appid)?;

    info.apply_env();

    let (program, args) = match command.split_first() {
        Some((p, a)) => (p, a),
        None => bail!("exec requires a command"),
    };

    let err = Command::new(program).args(args).exec();
    bail!("Failed to exec {}: {err}", program.to_string_lossy());
}
