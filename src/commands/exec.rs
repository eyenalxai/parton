use crate::process::spawn_and_wait;
use crate::proton::WineserverInfo;
use anyhow::{Result, bail};
use std::ffi::OsString;
use std::process::Command;

pub fn run(appid: &str, command: Vec<OsString>) -> Result<()> {
    let info = WineserverInfo::find_by_appid(appid)?;

    info.apply_env();

    let (program, args) = match command.split_first() {
        Some((p, a)) => (p, a),
        None => bail!("exec requires a command"),
    };

    let mut cmd = Command::new(program);
    cmd.args(args);
    spawn_and_wait(cmd)
}
