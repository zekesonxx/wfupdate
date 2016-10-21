use std::env;
use std::process::Command;
use std::path::PathBuf;
use config;

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
pub fn game_executable(gamedir: PathBuf) -> Command{
    let mut gamedir = gamedir.clone();
    let config = config::get();
    gamedir.push(if config::parse_bool(config.get_from(Some("game"), "64bit")) {"Warframe.x64.exe"} else {"Warframe.exe"});
    let mut cmd = Command::new(find_wine_binary());
    cmd.arg(gamedir.to_str().unwrap());
    cmd
}

#[cfg(unix)]
pub fn launcher_executable(launcherpath: PathBuf) -> Command {
    let mut cmd = Command::new(find_wine_binary());
    cmd.arg(launcherpath.to_str().unwrap());
    cmd
}

pub fn base_game_command(gamedir: PathBuf) -> Command {
    let mut cmd = game_executable(gamedir);
    let config = config::get();
    cmd.args(&[
        "-log:/wfupdate.log",
        "-threadedworker:1",
        "-cluster:public",
    ]);
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
    cmd.arg("-silent").arg("-applet:/EE/Types/Framework/CacheRepair");
    cmd
}

// "C:\Program Files\Warframe\Downloaded\Public\Warframe.exe" -dx10:0 -dx11:0 -threadedworker:1 -cluster:public -language:en -fullscreen:0
pub fn build_game_run(gamedir: PathBuf) -> Command {
    let mut cmd = base_game_command(gamedir);
    cmd.arg("-fullscreen:0");
    cmd
}
