use std::env;
use std::process::Command;
use std::path::PathBuf;
use config;
use paths;
use time;

pub fn find_wine_binary() -> PathBuf {
    match env::var("WINE").or(env::var("WARFRAMEWINE")) {
        Ok(winepath) => {
            let winepath = PathBuf::from(winepath);
            if winepath.metadata().is_ok() {
                return winepath;
            }
        },
        Err(_) => {}
    }
    PathBuf::from("/usr/bin/wine")
}

#[cfg(unix)]
pub fn wine_cmd() -> Command {
    let config = config::get();
    // First, we need the Wine binary path
    let wine = {
        let is64bit = config::parse_bool(config.get_from(Some("game"), "64bit"));
        if let Some(winebin) = config.get_from(Some("wine"), "winebin") {
            let mut path = PathBuf::from(winebin);
            path.push(if is64bit {"wine64"} else {"wine"});
            path
        } else {
            match env::var("WINE") {
                Ok(winepath) => {
                    let winepath = PathBuf::from(winepath);
                    if winepath.metadata().is_ok() {
                        winepath
                    } else {
                        PathBuf::from("wine")
                    }
                },
                Err(_) => PathBuf::from("wine")
            }
        }
    };
    let mut cmd = Command::new(wine);
    cmd.env("WINEPREFIX", paths::wine::wineprefix().as_os_str());
    cmd.env("WINEARCH", config.get_from(Some("wine"), "winearch").unwrap_or("win32"));
    if let Some(ldpath) = config.get_from(Some("wine"), "winelib") {
        cmd.env("LD_LIBRARY_PATH", format!("{} {}", ldpath, env::var("LD_LIBRARY_PATH").unwrap_or(String::new())));
    }
    cmd
}

#[cfg(unix)]
pub fn game_executable(gamedir: PathBuf) -> Command{
    let mut gamedir = gamedir.clone();
    let config = config::get();
    gamedir.push(if config::parse_bool(config.get_from(Some("game"), "64bit")) {"Warframe.x64.exe"} else {"Warframe.exe"});
    let mut cmd = wine_cmd();
    cmd.arg(gamedir.to_str().unwrap());
    cmd
}

#[cfg(unix)]
pub fn launcher_executable(launcherpath: PathBuf) -> Command {
    let mut cmd = wine_cmd();
    cmd.arg(launcherpath.to_str().unwrap());
    cmd
}

pub fn base_game_command(gamedir: PathBuf) -> Command {
    let mut cmd = game_executable(gamedir);
    let config = config::get();
    cmd.args(&[
        "-threadedworker:1",
        "-cluster:public",
    ]);
    cmd.arg(if config::parse_bool(config.get_from(Some("game"), "logtime")) {
                format!("-log:/wfupdate-{}.log", time::now().strftime("%s").unwrap())
            } else {
                "-log:/wfupdate.log".to_string()
            });
    cmd.arg(match config.get_from(Some("game"), "dx10") {
        Some("true") | Some("1") => "-dx10:1",
        None | _ => "-dx10:0",
    });
    cmd.arg(match config.get_from(Some("game"), "dx11") {
        Some("true") | Some("1") => "-dx11:1",
        None | _ => "-dx11:0",
    });
    cmd.arg(format!("-language:{}", config.get_from(Some("game"), "language").unwrap_or("en")));
    cmd
}

// "C:\Program Files\Warframe\Downloaded\Public\Warframe.exe" -silent -log:/Preprocess.log -dx10:0 -dx11:0 -threadedworker:1 -cluster:public -language:en -applet:/EE/Types/Framework/ContentUpdate
pub fn build_game_update(gamedir: PathBuf) -> Command {
    let mut cmd = base_game_command(gamedir);
    cmd.arg("-applet:/EE/Types/Framework/ContentUpdate");
    cmd
}

pub fn build_game_repair(gamedir: PathBuf) -> Command {
    let mut cmd = base_game_command(gamedir);
    cmd.arg("-applet:/EE/Types/Framework/CacheRepair");
    cmd
}

pub fn build_game_defrag(gamedir: PathBuf) -> Command {
    let mut cmd = base_game_command(gamedir);
    cmd.arg("-applet:/EE/Types/Framework/CacheDefragger");
    cmd
}

// "C:\Program Files\Warframe\Downloaded\Public\Warframe.exe" -dx10:0 -dx11:0 -threadedworker:1 -cluster:public -language:en -fullscreen:0
pub fn build_game_run(gamedir: PathBuf) -> Command {
    let mut cmd = base_game_command(gamedir);
    cmd.arg("-fullscreen:0");
    cmd
}
