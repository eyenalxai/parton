use crate::proton::{get_game_name, scan_running_games};
use anyhow::Result;

pub fn run() -> Result<()> {
    for info in scan_running_games() {
        let name = get_game_name(&info.compatdata, &info.appid);
        println!("{}\t{}", info.appid, name);
    }
    Ok(())
}
