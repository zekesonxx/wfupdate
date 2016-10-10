#![allow(dead_code)]

use std::env;
use std::process::Command;
use std::path::PathBuf;
use std::collections::HashMap;

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

fn get_wine_version(wine: &PathBuf) -> Option<String> {
    match Command::new(wine).arg("--version").output() {
        Ok(output) => {
            match String::from_utf8(output.stdout) {
                Ok(s) => Some(s.trim_right().split_at(5).1.to_string()),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub fn build_wine_versions_list() -> HashMap<String, PathBuf> {
    let mut wines = HashMap::new();
    macro_rules! trywine {
        ($path: expr) => (
            let path = $path;
            match get_wine_version(&path) {
                Some(version) => {wines.insert(version, path);},
                None => {}
            }
        )
    }
    trywine!(PathBuf::from("/usr/bin/wine"));
    if let Some(homedir) = env::home_dir() {
        //Let's check PoL
        let mut pol: PathBuf = homedir.clone();
        pol.push(".PlayOnLinux");
        pol.push("wine");
        pol.push("linux-x86");
        if pol.metadata().is_ok() {
            //PoL exists, let's see what it has.
            let iter = pol.read_dir();
            if iter.is_ok() {
                for dir in iter.unwrap() {
                    if let Ok(metadata) = dir {
                        let mut path = pol.clone();
                        path.push(metadata.path());
                        path.push("bin");
                        path.push("wine");
                        trywine!(path);
                    }
                }
            }
        }
    }
    wines
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
