#[macro_use] extern crate lazy_static;
#[macro_use] extern crate clap;
extern crate bytesize;
pub mod logparser;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use logparser::LogLine;
use bytesize::ByteSize;
use clap::App;


fn percentage(amount: u64, total: u64) -> String {
    if total == 0 {
        return String::from("");
    }
    let frac: f64 = (amount*100u64) as f64/(total*100u64) as f64;

    let mut output = format!("{}", frac*100f64);
    output.truncate(5);
    output
}

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Create a path to the desired file
    let path = Path::new(matches.value_of("INPUT").unwrap_or("Preprocess.log"));
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file;
    match File::open(&path) {
        Err(_) => {
            println!("couldn't open {}, see --help for help", display);
            return;
        },
        Ok(handle) => {
            file = handle
        },
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => {}
    }

    let mut total_bytes: u64 = 0;
    let mut downloaded_bytes: u64 = 0;
    let mut total_files: u64 = 0;
    let mut downloaded_files: u64 = 0;
    let parsed = logparser::parse_lines(s.as_str());
    for result in parsed {
        match result {
            LogLine::HashMismatch(_) => {
                total_files += 1;
            },
            LogLine::BytesToDownload(bytes) => {
                total_bytes = bytes;
            },
            LogLine::UsedShared(size, _) => {
                downloaded_bytes += size;
                downloaded_files += 1;
            },
            LogLine::Unknown(_) => {}
        }
    }

    let bytes = format!("bytes: {}/{} {}%", ByteSize::b(downloaded_bytes as usize), ByteSize::b(total_bytes as usize), percentage(downloaded_bytes, total_bytes));
    let filecount = format!("files: {}/{} {}%", downloaded_files, total_files, percentage(downloaded_files, total_files));
    println!("{}; {}", bytes, filecount);

}
