//! Warframe Stage 1 Updating Module
//!
//! This module is responsible for checking for, downloading, and applying updates.
//!
//! You can see more about the update process and Stage 1/Stage 2 in `LAUNCHERPROTOCOL.md`.
//!
#![deny(missing_docs)]
pub mod parser;
pub mod downloader;
pub mod checker;
pub mod update;
pub use self::parser::FileType;


/// A Warframe File parsed out of the Launcher index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    /// The full path to download the asset from Warframe servers
    ///
    /// ex `/Warframe.exe.81E34ABBF3AEFAD7E56D157EDE08E178.lzma`
    pub download_path: String,
    /// The on-disk path (the download_path without the `.<md5sum>.lzma`) suffix
    ///
    /// ex `/Warframe.exe`
    pub disk_path: String,
    /// The MD5 hash from the download_path, as a u8 vector.
    pub md5sum: Vec<u8>,
    /// The size of the compressed file, in bytes
    pub size: u64
}
