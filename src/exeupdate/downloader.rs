use rand::Rng;
use lzma;
use hyper;
use hyper::Client;
use std::io::Read;

static WARFRAME_CONTENT_URL: &'static str = "http://origin.warframe.com/";

pub fn build_index_url() -> String {
    let mut rng = ::rand::thread_rng();
    format!("http://origin.warframe.com/origin/{:08X}/index.txt.lzma", rng.gen::<u32>())
}

#[derive(Debug)]
pub enum DownloaderError {
    LZMAError(lzma::Error),
    HyperError(hyper::Error),
    IOError(::std::io::Error)
}

pub fn get_index() -> Result<String, DownloaderError> {
    let mut rng = ::rand::thread_rng();
    match get_file(format!("origin/{:08X}/index.txt.lzma", rng.gen::<u32>())) {
        Ok(body) => Ok(String::from_utf8(body).unwrap()),
        Err(e) => Err(e)
    }
}


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
