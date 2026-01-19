use crate::process::spawn_and_wait_wine;
use crate::proton::WineserverInfo;
use anyhow::Result;
use std::ffi::OsString;
use std::path::PathBuf;

pub fn run(
    appid: &str,
    exe: PathBuf,
    args: Vec<OsString>,
    bypass_gamescope: Option<String>,
) -> Result<()> {
    let info = WineserverInfo::find_by_appid(appid)?;
    let cmd = info.wine_command(exe.as_os_str(), &args, bypass_gamescope.as_deref())?;

    let exe_name = exe.file_name().and_then(|n| n.to_str());

    spawn_and_wait_wine(cmd, Some(&info.wine64), exe_name)
}
