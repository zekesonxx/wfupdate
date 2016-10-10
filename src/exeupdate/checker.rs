use super::super::paths;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use crypto::md5::Md5;
use crypto::digest::Digest;

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


pub fn check_file(file: &super::File) -> io::Result<bool> {
    let mut file = file.clone();
    file.disk_path = match paths::realize_path(file.disk_path) {
        Some(p) => p.to_str().unwrap().to_string(),
        None => return Ok(true)
    };
    direct_check_file(&file)
}


pub fn check_files(files: Vec<super::File>) -> io::Result<Vec<super::File>> {
    let mut needs_update = vec![];
    for file in files {
        if file.disk_path.contains("x64") || file.disk_path.contains("Language") || file.disk_path.contains("Cache.Windows") {
            continue;
        }
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
