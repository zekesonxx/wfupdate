use std::path::PathBuf;

#[cfg(unix)]
pub mod wine;
#[cfg(unix)]
pub use self::wine as plat;

macro_rules! optiontry {
    ($a: expr) => (match $a {
        Some(k) => k,
        None => return None
    })
}

/// Finds the directory where the official Launcher resides (where the logs are)
///
/// Usually, something like `C:/Users/<username>/Local Settings/Application Data/Warframe/`
///
/// The actual Launcher executable can be found in subdirectories of this directory, usually `Downloaded/Public/Tools/Launcher.exe`
#[inline(always)]
pub fn launcher_dir() -> Option<PathBuf> {
    self::plat::launcher_dir()
}

pub fn launcher_exe() -> Option<PathBuf> {
    let mut launcher_path = optiontry!(self::plat::launcher_dir());
    launcher_path.push("Downloaded");
    launcher_path.push("Public");
    launcher_path.push("Tools");
    launcher_path.push("Launcher.exe");
    if launcher_path.metadata().is_ok() {
        Some(launcher_path)
    } else {
        None
    }
}

/// Finds the directory where the game is installed (where `Warframe.exe` is)
///
/// Usually, something like `C:/Program Files/Warframe/Downloaded/Public`
#[inline(always)]
pub fn game_install_dir() -> Option<PathBuf> {
    self::plat::game_install_dir()
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
