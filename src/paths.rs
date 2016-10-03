

use users::get_current_username;
use std::path::PathBuf;
use std::env;

pub fn guess_log_folder_from_wineprefix() -> Option<PathBuf> {
    match env::var("WINEPREFIX") {
        Ok(wineprefix) => {
            let mut maybedir = PathBuf::from(wineprefix);
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
        },
        Err(_) => None
    }
}


pub fn guess_game_install_dir_from_wineprefix() -> Option<PathBuf> {
    match env::var("WINEPREFIX") {
        Ok(wineprefix) => {
            let mut maybedir = PathBuf::from(wineprefix);
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
        },
        Err(_) => None
    }
}
