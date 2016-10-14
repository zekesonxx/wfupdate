//! Warframe File Checker
//!
//! This file is responsible for checking each file, parsed out of the Launcher index, and seeing if it needs to be updated.

use super::super::paths;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use crypto::md5::Md5;
use crypto::digest::Digest;

/// Directly check a file, bypassing path realization.
///
/// You shouldn't ever need to use this directly.
pub fn direct_check_file(file: &super::File) -> io::Result<bool> {
    let f = match File::open(file.disk_path.clone()) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                return Ok(true);
            } else {
                return Err(e);
            }
        }
    };
    let mut md5 = Md5::new();
    let mut reader = BufReader::new(f);

    let mut buf = vec![];
    reader.read_to_end(&mut buf);
    md5.input(buf.as_slice());

    let mut result: [u8; 16] = [0; 16];
    md5.result(&mut result);

    Ok(result != file.md5sum.as_slice())
}

/// Check if a file needs to be updated.
///
/// This function will automatically realize paths, for example:
/// `/Warframe.exe` will be realized to `$PROGRAMFILES/Warframe/Downloaded/Public/Warframe.exe`<br/>
/// or<br/>
/// `/Tools/Launcher.exe` will be realized to
/// `/users/$USERNAME/Local Settings/Application Data/Warframe/Downloaded/Public/Tools/Launcher.exe`
///
/// (see more about path realization in `paths::realize_path()`)
///
/// This function performs file I/O and will be MD5 hashing the file in question.
pub fn check_file(file: &super::File) -> io::Result<bool> {
    let mut file = file.clone();
    file.disk_path = match paths::realize_path(file.disk_path) {
        Some(p) => p.to_str().unwrap().to_string(),
        None => return Ok(true)
    };
    direct_check_file(&file)
}


/// Conveience function to loop over a File vector.
///
/// Returns a vector of the files that need to be updated
pub fn check_files(files: Vec<super::File>) -> io::Result<Vec<super::File>> {
    let mut needs_update = vec![];
    for file in files {
        match check_file(&file) {
            Err(e) => return Err(e),
            Ok(to_update) => {
                if to_update {
                    needs_update.push(file);
                }
            }
        }
    }
    Ok(needs_update)
}
