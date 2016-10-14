//! Warframe Launcher Index File Parser
//!
//! This file is responsible for parsing a big list of files into a computer-usable format.
//!
//! You can find a more in-depth look at the launcher update protocol in `LAUNCHERPROTOCOL.md`.
//!
//! Check out the documentation for `parse_file_list()` for more info.

use hex::FromHex;
use super::File;

/// Parses a Warframe file index list into a vector of `File`s.
///
/// The list looks something like this:
///
/// ```
/// /Tools/Launcher.exe.F336FD22FDF21024C75FF46FE8F7A06E.lzma,313612
/// /Tools/Windows/x86/msvcr110.dll.4BA25D2CBE1587A841DCFB8C8C4A6EA6.lzma,351188
/// /Tools/Windows/x86/steam_api.dll.A83ADE32811F1419685E90F592ADF505.lzma,77655
/// /Tools/Windows/x86/symsrv.dll.64DEA54A4457371DEC27A4CFAE6EFB50.lzma,47951
/// /Warframe.exe.3BB594902B2E8037901ED9B2419E8FD5.lzma,6998205
/// ```
///
/// ## Example
/// ```rust
/// # use super::File;
/// assert_eq!(parse_file_list("/Tools/Launcher.exe.F336FD22FDF21024C75FF46FE8F7A06E.lzma,313612".to_string()).unwrap(),
/// File {
///     download_path: "/Tools/Launcher.exe.F336FD22FDF21024C75FF46FE8F7A06E.lzma,313612".to_string(),
///     disk_path: "/Tools/Launcher.exe",
///     md5sum: vec![0xF3, 0x36, 0xFD, 0x22, 0xFD, 0xF2, 0x10, 0x24, 0xC7, 0x5F, 0xF4, 0x6F, 0xE8, 0xF7, 0xA0, 0x6E],
///     size: 313612
/// });
/// ```
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
        let md5: Vec<u8> = match Vec::from_hex(raw_md5.as_bytes()) {
            Ok(k) => k,
            Err(_) => return Err(())
        };

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

/// A categorization of a Warframe asset.
///
/// Used to decide if it should be checked and possibly updated or not.
#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    /// 32-Bit Executable. This should always be present.
    ///
    /// Also includes things like language files and driver redistributables.
    Exe32Bit,
    /// 64-Bit Executable. Only needed if you're running the game in 64-bit mode.
    Exe64Bit,
    /// Very large game asset that isn't managed by the launcher.
    ///
    /// I don't know why these are in the list.
    GameAsset,
    /// Asset for the vanilla game Launcher
    ///
    /// As far as I'm aware these aren't required for the game to run.
    LauncherAsset,
    /// Asset needed for running the game installed via Steam
    ///
    /// The game will silently not care if you don't have these present.
    SteamAsset,
    /// An unknown file.
    ///
    /// As of 2016-10-13, there are no items in the list that aren't categorized.
    Unknown
}


/// Categorizes a Warframe file asset
///
/// Paths should be how they are in the file list, ex
/// "/Warframe.exe" not "Program Files/Warframe/Downloaded/Public/Warframe.exe"
///
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


#[cfg(test)]
mod tests {
    use super::*;
    use super::FileType::*;
    macro_rules! quickcat {
        ($path: expr) => (
            categorize(&super::super::File {
                disk_path: $path.to_string(),
                download_path: "".to_string(),
                md5sum: vec![],
                size: 0
            })
        )
    }

    #[test]
    pub fn test_categorize() {
        assert_eq!(quickcat!("/Warframe.exe"), Exe32Bit);
        assert_eq!(quickcat!("/Warframe.x64.exe"), Exe64Bit);
        assert_eq!(quickcat!("/Cache.Windows/B.AnimRetarget.cache"), GameAsset);
        assert_eq!(quickcat!("/Tools/Windows/steamController.vdf"), SteamAsset);
        assert_eq!(quickcat!("/Tools/CEF3_1/launcher.zip"), LauncherAsset);
        assert_eq!(quickcat!("/etc/passwd"), Unknown);
    }

    #[test]
    pub fn test_parse() {
        // This is the nicest way I could think of doing this that didn't involve including another file.
        // Newlines are CRLF instead of LF because that's what the real file has because Windows is stupid.
        let mut test_list: String = "/Tools/Launcher.exe.F336FD22FDF21024C75FF46FE8F7A06E.lzma,313612\r\n".to_string();
        test_list.push_str("/Tools/Windows/x86/msvcr110.dll.4BA25D2CBE1587A841DCFB8C8C4A6EA6.lzma,351188\r\n");
        test_list.push_str("/Tools/Windows/x86/steam_api.dll.A83ADE32811F1419685E90F592ADF505.lzma,77655\r\n");
        test_list.push_str("/Tools/Windows/x86/symsrv.dll.64DEA54A4457371DEC27A4CFAE6EFB50.lzma,47951\r\n");
        test_list.push_str("/Warframe.exe.3BB594902B2E8037901ED9B2419E8FD5.lzma,6998205\r\n");

        // We're not doing a complete test here
        // In the future there will be an integration test over a full index.txt.
        let result = parse_file_list(test_list);
        assert!(result.is_ok());

        // bad filesize
        let result = parse_file_list("/Warframe.exe.3BB594902B2E8037901ED9B2419E8FD5.lzma,dskfn1j3r\r\n".to_string());
        assert!(result.is_err());

        // invalid MD5 hash
        let result = parse_file_list("/Warframe.exe.CATSCATSCATS8037901ED9B2419E8FD5.lzma,6998205\r\n".to_string());
        assert!(result.is_err());
    }
}
