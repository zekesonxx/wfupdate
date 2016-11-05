#[macro_use] extern crate lazy_static;
#[macro_use] extern crate clap;
extern crate bytesize;
extern crate users;
extern crate hyper;
extern crate rand;
extern crate lzma;
extern crate crypto;
extern crate hex;
extern crate xdg;
extern crate ini;
pub mod logparser;
pub mod paths;
pub mod wine;
pub mod exeupdate;
pub mod config;
pub mod cli;
pub mod run;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::path::PathBuf;
use logparser::LogLine;
use bytesize::ByteSize;
use clap::App;

// Update:
// "C:\Program Files\Warframe\Downloaded\Public\Warframe.exe" -silent -log:/Preprocess.log -dx10:0 -dx11:0
// -threadedworker:1 -cluster:public -language:en -applet:/EE/Types/Framework/ContentUpdate
// Run:
// "C:\Program Files\Warframe\Downloaded\Public\Warframe.exe" -dx10:0 -dx11:0 -threadedworker:1 -cluster:public -language:en -fullscreen:0

fn percentage(amount: u64, total: u64) -> String {
    if total == 0 {
        return String::from("");
    }
    let frac: f64 = (amount*100u64) as f64/(total*100u64) as f64;

    let mut output = format!("{}", frac*100f64);
    output.truncate(5);
    output
}

fn display_parsed(parsed: &Vec<LogLine>) {
    let mut total_bytes: u64 = 0;
    let mut downloaded_bytes: u64 = 0;
    let mut total_files: u64 = 0;
    let mut downloaded_files: u64 = 0;
    for result in parsed {
        match result {
            &LogLine::HashMismatch(_) => {
                total_files += 1;
            },
            &LogLine::BytesToDownload(bytes) => {
                total_bytes = bytes;
            },
            &LogLine::UsedShared(size, _) => {
                downloaded_bytes += size;
                downloaded_files += 1;
            },
            &LogLine::Unknown(_) => {}
        }
    }

    let bytes = format!("bytes: {}/{} {}%", ByteSize::b(downloaded_bytes as usize), ByteSize::b(total_bytes as usize), percentage(downloaded_bytes, total_bytes));
    let filecount = format!("files: {}/{} {}%", downloaded_files, total_files, percentage(downloaded_files, total_files));
    println!("{}; {}", bytes, filecount);
}


fn parse_file(path: PathBuf) {
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file;
    match File::open(&path) {
        Err(_) => {
            println!("couldn't open {}, see --help for help", display);
            exit(1);
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

    let parsed = logparser::parse_lines(s.as_str());
    display_parsed(&parsed);
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
                       .subcommand(cli::run::subcommand())
                       .subcommand(cli::config::subcommand())
                       .subcommand(cli::wine::subcommand())
                       .subcommand(cli::update::subcommand())
                       .subcommand(cli::repair::subcommand())
                       .get_matches();

    if let Some(matches) = matches.subcommand_matches("parse") {
        // Create a path to the desired file
        let path = match matches.value_of("INPUT") {
            Some(p) => PathBuf::from(p),
            None => match paths::launcher_dir() {
                Some(mut p) => {
                    p.push("Preprocess.log");
                    p
                },
                None => PathBuf::from("Preprocess.log")
            }
        };
        parse_file(path);
        return;
    } else if let Some(matches) = matches.subcommand_matches("wine") {
        cli::wine::run(matches);
    } else if let Some(matches) = matches.subcommand_matches("update") {
        cli::update::run(matches);
    } else if let Some(matches) = matches.subcommand_matches("run") {
        cli::run::run(matches);
    } else if let Some(matches) = matches.subcommand_matches("config") {
        cli::config::run(matches);
    } else if let Some(matches) = matches.subcommand_matches("repair") {
        cli::repair::run(matches);
    }
}
