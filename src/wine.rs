#![allow(dead_code)]

use std::env;
use std::process::Command;
use std::path::PathBuf;

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
    build_game_update(PathBuf::from("cats"));
    PathBuf::from("/usr/bin/wine")
}

pub fn base_game_command(gamedir: PathBuf) -> Command {
    let mut gamedir = gamedir.clone();
    gamedir.push("Warframe.exe");
    let mut cmd = Command::new(find_wine_binary());
    cmd.args(&[
        gamedir.to_str().unwrap(),
        "-log:/wfupdate.log",
        "-dx10:0",
        "-dx11:0",
        "-threadedworker:1",
        "-cluster:public",
        "-language:en",
    ]);
    cmd
}

// "C:\Program Files\Warframe\Downloaded\Public\Warframe.exe" -silent -log:/Preprocess.log -dx10:0 -dx11:0 -threadedworker:1 -cluster:public -language:en -applet:/EE/Types/Framework/ContentUpdate
pub fn build_game_update(gamedir: PathBuf) -> Command {
    let mut cmd = base_game_command(gamedir);
    cmd.arg("-silent").arg("-applet:/EE/Types/Framework/ContentUpdate");
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
