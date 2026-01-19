mod game;
mod wineserver;

pub use game::get_game_name;
pub use wineserver::{scan_running_games, WineserverInfo};
