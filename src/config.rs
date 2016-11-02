use xdg::BaseDirectories;
use ini::Ini;
use std::path::PathBuf;

lazy_static! {
    static ref XDG_BASEDIR: BaseDirectories = BaseDirectories::with_prefix("wfupdate").unwrap();
    static ref CONFIG_FILE_PATH: PathBuf = XDG_BASEDIR.place_config_file("config.ini").unwrap();
}

pub fn get() -> Ini {
    Ini::load_from_file(CONFIG_FILE_PATH.to_str().unwrap()).unwrap()
}

pub fn set(input: Ini) {
    let _ = input.write_to_file(CONFIG_FILE_PATH.to_str().unwrap());
}


/// Parses a human-readable config key into a computer-friendly string
///
/// # Examples
/// ```rust
/// parse_configid("wine:wineprefix"); //(Some("wine"), "wineprefix")
/// parse_configid("game:dx11"); //(Some("game"), "dx11")
/// parse_configid("encoding"); //(None, "encoding")
/// ```
pub fn parse_configid(input: &str) -> (Option<String>, String) {
    match input.find(':') {
        Some(pos) => {
            let slice = input.split_at(pos);
            // .split_at(1).1 is a cheap trick to remove the first character of a string
            return (Some(slice.0.to_string()), slice.1.split_at(1).1.trim_left_matches(':').to_string());
        },
        None => {
            return (None, String::from(input));
        }
    }
}

pub fn parse_bool(input: Option<&str>) -> bool {
    match input {
        Some("true") | Some("1") => true,
        None | _ => false,
    }
}
