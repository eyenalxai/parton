use clap::builder::StyledStr;
use clap_complete::engine::CompletionCandidate;
use std::ffi::OsStr;

use crate::steam::{Steam, get_game_name};
use crate::wineserver::scan_running_games;

fn matches_prefix(current: &OsStr, value: &str) -> bool {
    current
        .to_str()
        .is_some_and(|prefix| value.starts_with(prefix))
}

pub fn complete_installed_appid(current: &OsStr) -> Vec<CompletionCandidate> {
    let Ok(steam) = Steam::new(None) else {
        return Vec::new();
    };
    let Ok(games) = steam.list_proton_games() else {
        return Vec::new();
    };

    games
        .into_iter()
        .filter(|(appid, _, _)| matches_prefix(current, appid))
        .map(|(appid, name, _)| CompletionCandidate::new(appid).help(Some(StyledStr::from(name))))
        .collect()
}

pub fn complete_running_appid(current: &OsStr) -> Vec<CompletionCandidate> {
    scan_running_games()
        .into_iter()
        .filter(|info| matches_prefix(current, &info.appid))
        .map(|info| {
            let name = get_game_name(&info.compatdata, &info.appid);
            CompletionCandidate::new(info.appid).help(Some(StyledStr::from(name)))
        })
        .collect()
}

pub fn complete_user_id(current: &OsStr) -> Vec<CompletionCandidate> {
    let Ok(steam) = Steam::new(None) else {
        return Vec::new();
    };
    let Ok(users) = steam.list_users() else {
        return Vec::new();
    };

    users
        .into_iter()
        .filter(|(user_id, _)| matches_prefix(current, user_id))
        .map(|(user_id, persona_name)| {
            CompletionCandidate::new(user_id).help(persona_name.map(StyledStr::from))
        })
        .collect()
}
