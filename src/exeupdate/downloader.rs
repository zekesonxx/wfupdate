//! Warframe Asset Downloader
//!
//! Handles getting and decompressing data from Warframe content servers.
//!
//! `get_file()` is the main function here.


use rand::Rng;
use lzma;
use hyper;
use hyper::Client;
use std::io::Read;

static WARFRAME_CONTENT_URL: &'static str = "http://origin.warframe.com/";

/// A general purpose error type for get_file.
///
/// To understand what will throw this error, read the source of `get_file()`.
#[derive(Debug)]
pub enum DownloaderError {
    /// Error circa hyper (failed to download, etc)
    HyperError(hyper::Error),
    /// Error circa lzma (failed to decode, etc)
    LZMAError(lzma::Error),
    /// IO error. Currently can only occur from lzma.
    IOError(::std::io::Error)
}

/// Gets the Warframe launcher file list and parses it into a String.
///
/// This is just a conveience function to calling `get_file` with the right URL
/// and then `String::from_utf8`ing it.
///
/// The URL in question is `http://origin.warframe.com/origin/XXXXXXXX/index.txt.lzma`,
/// where `XXXXXXXX` is eight random hexadecimal characters.
pub fn get_index() -> Result<String, DownloaderError> {
    let mut rng = ::rand::thread_rng();
    match get_file(format!("origin/{:08X}/index.txt.lzma", rng.gen::<u32>())) {
        Ok(body) => Ok(String::from_utf8(body).unwrap()),
        Err(e) => Err(e)
    }
}

/// Retrieves a file from Warframe servers, decompresses it, and returns it as a u8 vector.
///
/// ## Example
/// ```rust,no_run
/// let result = get_file("/Tools/Launcher.exe.F336FD22FDF21024C75FF46FE8F7A06E.lzma".to_string()).unwrap();
/// ```
/// This will issue a request for `http://origin.warframe.com/Tools/Launcher.exe.F336FD22FDF21024C75FF46FE8F7A06E.lzma`,
/// download the file, LZMA decode it, and return it as a Vec<u8>.
pub fn get_file(path: String) -> Result<Vec<u8>, DownloaderError> {
    let client = Client::new();
    let res = match client.get(format!("{}{}", WARFRAME_CONTENT_URL, path).as_str()).send() {
        Ok(res) => res,
        Err(err) => return Err(DownloaderError::HyperError(err))
    };
    let mut body = vec![];

    let mut decoder = match lzma::read(res) {
        Ok(decoder) => decoder,
        Err(err) => return Err(DownloaderError::LZMAError(err))
    };
    if let Err(err) = decoder.read_to_end(&mut body) {
        return Err(DownloaderError::IOError(err));
    }

    Ok(body)
}
