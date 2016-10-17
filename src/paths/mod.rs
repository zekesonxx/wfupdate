use users::get_current_username;
use std::path::PathBuf;
use std::env;

#[cfg(unix)]
pub mod wine;
#[cfg(unix)]
pub use self::wine as plat;


/// Finds the directory where the official Launcher resides (where the logs are)
///
/// Usually, something like `C:/Users/<username>/Local Settings/Application Data/Warframe/`
///
/// The actual Launcher executable can be found in subdirectories of this directory, usually something like `Downloaded/Public/Tools/Launcher.exe`
#[inline(always)]
pub fn launcher_dir() -> Option<PathBuf> {
    self::plat::launcher_dir()
}

/// Finds the directory where the game is installed (where `Warframe.exe` is)
///
/// Usually, something like `C:/Program Files/Warframe/Downloaded/Public`
#[inline(always)]
pub fn game_install_dir() -> Option<PathBuf> {
    self::plat::game_install_dir()
}

macro_rules! optiontry {
    ($a: expr) => (match $a {
        Some(k) => k,
        None => return None
    })
}

pub fn realize_path(input: String) -> Option<PathBuf> {
    if input.starts_with("/Tools/CEF3_1") || input.starts_with("/Tools/Launcher.exe") {
        // Launcher file
        let mut out = match launcher_dir() {
            Some(path) => path,
            None => return None
        };
        out.push("Downloaded/Public/");
        out.push(input.split_at(1).1);
        Some(out)
    } else {
        // Game file
        let mut out = match game_install_dir() {
            Some(path) => path,
            None => return None
        };
        out.push(input.split_at(1).1);
        Some(out)
    }
}
