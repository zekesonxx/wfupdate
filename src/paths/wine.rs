//! Linux (and probably macOS) platform-specific path finding
//!

use users::get_current_username;
use std::path::PathBuf;
use std::env;
use super::super::config;

/// Figures out the user's wineprefix
///
/// This function looks in this order:
///
/// * Config variable: `wine:wineprefix`
/// * Environment variable: `WINEPREFIX`
/// * Default wineprefix: `$HOME/.wine/`
///
/// This function does not test if the directories exist or not, or if they are actually wineprefixes.
///
/// # Panics
/// Panics if the user doesn't have a home directory.
pub fn wineprefix() -> PathBuf {
    let config = config::get();
    // First try: wineprefix config var
    if let Some(path) = config.get_from(Some("wine"), "wineprefix") {
        return PathBuf::from(path);
    }
    // Next try: wineprefix env var
    if let Ok(path) = env::var("WINEPREFIX") {
        return PathBuf::from(path);
    }
    // If after all that no luck, go with the default wineprefix.
    let mut out = env::home_dir().unwrap();
    out.push(".wine");
    out
}

pub fn game_install_dir() -> Option<PathBuf> {
    let mut maybedir = wineprefix();
    if maybedir.metadata().is_err() {
        //Wineprefix doesn't actually exist
        return None;
    }

    // Build up the path to the game install dir
    maybedir.push("drive_c");
    maybedir.push("Program Files");
    maybedir.push("Warframe");
    maybedir.push("Downloaded");
    maybedir.push("Public");

    if maybedir.metadata().is_ok() {
        // Dir exists, let's go for it.
        Some(maybedir)
    } else {
        //And, after all that, no dice.
        return None;
    }
}

pub fn launcher_dir() -> Option<PathBuf> {
    let mut maybedir = wineprefix();
    if maybedir.metadata().is_err() {
        //Wineprefix doesn't actually exist
        return None;
    }

    // Build up the path to the log file dir
    maybedir.push("drive_c");
    maybedir.push("users");
    maybedir.push(get_current_username().unwrap());
    maybedir.push("Local Settings");
    maybedir.push("Application Data");
    maybedir.push("Warframe");

    if maybedir.metadata().is_ok() {
        // Dir exists, let's go for it.
        Some(maybedir)
    } else {
        //And, after all that, no dice.
        return None;
    }
}
