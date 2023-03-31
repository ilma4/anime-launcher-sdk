use std::path::PathBuf;

/// Timeout used by `anime_game_core::telemetry::is_disabled` to check acessibility of telemetry servers
pub const TELEMETRY_CHECK_TIMEOUT: Option<u64> = Some(3);

/// Get default launcher dir path
/// 
/// `$HOME/.local/share/anime-game-launcher`
#[inline]
pub fn launcher_dir() -> anyhow::Result<PathBuf> {
    Ok(std::env::var("XDG_DATA_HOME")
        .or_else(|_| std::env::var("HOME").map(|home| home + "/.local/share"))
        .map(|home| PathBuf::from(home).join("anime-game-launcher"))?)
}

/// Get default config file path
/// 
/// `$HOME/.local/share/anime-game-launcher/config.json`
#[inline]
pub fn config_file() -> anyhow::Result<PathBuf> {
    launcher_dir().map(|dir| dir.join("config.json"))
}
