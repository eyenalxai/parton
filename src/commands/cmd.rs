use crate::process::spawn_and_wait_wine;
use crate::proton::WineserverInfo;
use anyhow::Result;
use std::ffi::OsStr;

pub fn run(appid: &str, bypass_gamescope: Option<String>) -> Result<()> {
    let info = WineserverInfo::find_by_appid(appid)?;
    let cmd = info.wine_command(
        OsStr::new("cmd.exe"),
        &[] as &[&str],
        bypass_gamescope.as_deref(),
    )?;

    spawn_and_wait_wine(cmd, Some(&info.wine64), Some("cmd.exe"))
}
