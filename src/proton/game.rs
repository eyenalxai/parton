use std::fs;
use std::path::Path;

pub fn get_game_name(compatdata: &Path, appid: &str) -> String {
    let steamapps = match compatdata.parent().and_then(|p| p.parent()) {
        Some(p) => p,
        None => return "(unknown)".to_string(),
    };

    let manifest = steamapps.join(format!("appmanifest_{appid}.acf"));

    let content = match fs::read_to_string(&manifest) {
        Ok(c) => c,
        Err(_) => return "(unknown)".to_string(),
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("\"name\"") {
            let parts: Vec<&str> = trimmed.split('"').collect();
            if parts.len() >= 4 {
                return parts.get(3).copied().unwrap_or("(unknown)").to_string();
            }
        }
    }

    "(unknown)".to_string()
}
