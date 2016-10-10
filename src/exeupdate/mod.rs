pub mod parser;
pub mod downloader;
pub mod checker;
pub use self::parser::FileType;

#[derive(Debug, Clone)]
pub struct File {
    pub download_path: String,
    pub disk_path: String,
    pub md5sum: Vec<u8>,
    pub size: u64
}
