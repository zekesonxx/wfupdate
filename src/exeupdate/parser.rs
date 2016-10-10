use hex::FromHex;
use super::File;

pub fn parse_file_list(list: String) -> Result<Vec<File>, ()> {
    let mut out = vec![];
    for line in list.lines() {
        let pos = match line.rfind(',') {
            Some(p) => p,
            None => return Err(())
        };
        let (filename, size) = line.split_at(pos);
        let size = size.split_at(1).1; //simple trick to remove the first character of the string (the comma)
        //md5 hash is 32 chars long, plus 5 for ".lzma", equals 37
        let (disk_path, raw_md5) = filename.split_at(filename.len()-37);
        let disk_path = disk_path.split_at(disk_path.len()-1).0;
        let raw_md5: &str = raw_md5.split_at(32).0; //remove the .lzma at the end
        let md5: Vec<u8> = Vec::from_hex(raw_md5.as_bytes()).unwrap();

        out.push(File {
            download_path: filename.to_string(),
            disk_path: disk_path.to_string(),
            md5sum: md5,
            size: match size.parse() {
                Ok(s) => s,
                Err(_) => return Err(())
            }
        });
    }
    Ok(out)
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    Exe32Bit,
    Exe64Bit,
    GameAsset,
    LauncherAsset,
    SteamAsset,
    Unknown
}

pub fn categorize(item: &File) -> FileType {
    let ref name = item.disk_path;
    if name.starts_with("/Tools/CEF3_1") || name == "/Tools/Launcher.exe" {
        FileType::LauncherAsset
    } else if name.contains("steam_api") || name.contains("vdf") {
        FileType::SteamAsset
    } else if name.contains("x64") {
        FileType::Exe64Bit
    } else if name.contains("x86") ||
              name == "/Warframe.exe" ||
              name.starts_with("/Drivers") ||
              name.starts_with("/Tools") {
        FileType::Exe32Bit
    } else if name.starts_with("/Cache.Windows") ||
              name.starts_with("/Lotus") {
        FileType::GameAsset
    } else {
        FileType::Unknown
    }
}
