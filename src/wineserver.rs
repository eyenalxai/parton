use anyhow::{Result, bail};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

use crate::proton::find_wine_bin;

pub struct WineserverInfo {
    pub appid: String,
    pub compatdata: PathBuf,
    pub wine_bin: PathBuf,
    pub env: HashMap<String, String>,
}

impl WineserverInfo {
    pub fn find_by_appid(target_appid: &str) -> Result<Self> {
        for info in scan_running_games() {
            if info.appid == target_appid {
                return Ok(info);
            }
        }
        bail!("No running game with appid {target_appid}");
    }

    pub fn wine_command<I, S>(
        &self,
        exe: &OsStr,
        args: I,
        bypass_gamescope: Option<&str>,
    ) -> Result<Command>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        if !self.wine_bin.exists() {
            bail!("wine binary not found at {}", self.wine_bin.display());
        }

        let mut cmd = Command::new(&self.wine_bin);
        for var in [
            "WINEFSYNC",
            "WINEESYNC",
            "SteamAppId",
            "STEAM_COMPAT_DATA_PATH",
        ] {
            if let Some(val) = self.env.get(var) {
                cmd.env(var, val);
            }
        }
        cmd.env("WINEPREFIX", self.compatdata.join("pfx"));

        if let Some(res) = bypass_gamescope {
            let desktop_name = format!("prex{}", process::id());
            cmd.arg("explorer")
                .arg(format!("/desktop={desktop_name},{res}"))
                .arg(exe)
                .args(args);
        } else {
            cmd.arg(exe).args(args);
        }

        Ok(cmd)
    }

}

#[must_use]
pub fn scan_running_games() -> Vec<WineserverInfo> {
    let proc = match fs::read_dir("/proc") {
        Ok(dir) => dir,
        Err(_) => return Vec::new(),
    };

    let mut results = Vec::new();

    for entry in proc.flatten() {
        let file_name = entry.file_name();
        if !file_name
            .as_os_str()
            .as_encoded_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit())
        {
            continue;
        }

        let proc_path = entry.path();

        let comm = match fs::read_to_string(proc_path.join("comm")) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if comm.trim() != "wineserver" {
            continue;
        }

        let environ = match fs::read(proc_path.join("environ")) {
            Ok(e) => e,
            Err(_) => continue,
        };

        let env = parse_environ(&environ);

        let appid = match env.get("SteamAppId") {
            Some(id) if !id.is_empty() => id.clone(),
            _ => continue,
        };

        let compatdata = match env.get("STEAM_COMPAT_DATA_PATH") {
            Some(p) if !p.is_empty() => PathBuf::from(p),
            _ => continue,
        };

        let exe_link = proc_path.join("exe");
        let wineserver_path = match fs::read_link(&exe_link) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let wine_bin = match wineserver_path
            .parent()
            .and_then(find_wine_bin)
        {
            Some(path) => path,
            None => continue,
        };

        results.push(WineserverInfo {
            appid,
            compatdata,
            wine_bin,
            env,
        });
    }

    results
}

fn parse_environ(data: &[u8]) -> HashMap<String, String> {
    let mut env = HashMap::new();
    for chunk in data.split(|&b| b == 0) {
        if let Ok(s) = std::str::from_utf8(chunk)
            && let Some((key, val)) = s.split_once('=')
        {
            env.insert(key.to_string(), val.to_string());
        }
    }
    env
}
