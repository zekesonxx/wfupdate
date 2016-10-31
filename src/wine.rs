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
    PathBuf::from("/usr/bin/wine")
}

fn get_wine_version(wine: &PathBuf) -> Option<String> {
    let mut wine = wine.clone();
    wine.push("bin/wine");
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

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
pub enum WineSource {
    System,
    PlayOnLinux
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct WineVersion {
    pub version: String,
    pub is_staging: bool,
    pub source: WineSource,
    pub path: PathBuf
}

impl WineVersion {
    pub fn new(version: String, source: WineSource, path: PathBuf) -> Self {
        let is_staging = match version.find(" (Staging)") {
            Some(_) => true,
            None => false
        };
        WineVersion {
            version: version,
            is_staging: is_staging,
            source: source,
            path: path
        }
    }
}

pub fn build_wine_versions_list() -> Vec<WineVersion> {
    let mut wines = vec![];
    macro_rules! trywine {
        ($path: expr, $source: expr) => (
            let path = $path;
            match get_wine_version(&path) {
                Some(version) => {wines.push(WineVersion::new(version, $source, path));},
                None => {}
            }
        )
    }
    trywine!(PathBuf::from("/usr"), WineSource::System);
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
                        trywine!(path, WineSource::PlayOnLinux);
                    }
                }
            }
        }
    }
    wines.sort_by_key(|ref i| i.version.clone());
    wines
}
