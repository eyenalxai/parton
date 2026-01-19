use crate::proton::WineserverInfo;
use anyhow::{bail, Result};
use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

pub fn run(
    appid: &str,
    exe: PathBuf,
    args: Vec<OsString>,
    bypass_gamescope: Option<String>,
) -> Result<()> {
    let info = WineserverInfo::find_by_appid(appid)?;

    if !info.wine64.exists() {
        bail!("wine64 not found at {}", info.wine64.display());
    }

    info.apply_env();

    let mut cmd = Command::new(&info.wine64);

    if let Some(res) = bypass_gamescope {
        cmd.arg("explorer")
            .arg(format!("/desktop=parton,{res}"))
            .arg(&exe)
            .args(&args);
    } else {
        cmd.arg(&exe).args(&args);
    }

    let err = cmd.exec();
    bail!("Failed to exec wine64: {err}");
}
