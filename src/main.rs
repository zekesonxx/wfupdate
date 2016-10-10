#[macro_use] extern crate lazy_static;
#[macro_use] extern crate clap;
extern crate bytesize;
extern crate users;
extern crate hyper;
extern crate rand;
extern crate lzma;
extern crate crypto;
extern crate hex;
pub mod logparser;
pub mod paths;
pub mod wine;
pub mod exeupdate;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Stdio, exit};
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

fn update_game(wfpath: PathBuf) {
    let mut parsed: Vec<LogLine> = vec![];
    let mut program = match wine::build_game_update(wfpath)
    .stdout(Stdio::piped())
    .spawn() {
        Ok(child) => child,
        Err(_) => {
            println!("Cannot run Warframe to update");
            return;
        },
    };
    match program.stdout.as_mut() {
        Some(out) => {
            let buf_reader = BufReader::new(out);
            println!("got bufreader");
            for line in buf_reader.lines() {
                match line {
                    Ok(l) => {
                        println!("{}", l);
                        let parsedline = logparser::parse_line(l.as_str());
                        let mut parse = true;
                        if let LogLine::Unknown(_) = parsedline {
                            parse = false;
                        } else if let LogLine::UsedShared(_, ref name) = parsedline {
                            if name.ends_with(".bin") {
                                parse = false;
                            }
                        }
                        if parse {
                            parsed.push(parsedline);
                            display_parsed(&parsed);
                        }
                    },
                    Err(_) => return,
                };
            }
        },
        None => return,
    }
}

fn run_game(wfpath: PathBuf) -> ! {
    use std::os::unix::process::CommandExt;
    let mut program = wine::build_game_run(wfpath);
    program.exec();
    panic!("Couldn't run Warframe");
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

fn checkupdate(matches: &clap::ArgMatches) {
    use exeupdate::FileType::*;
    let index = match exeupdate::downloader::get_index() {
        Ok(index) => index,
        Err(_) => {
            println!("Failed to get Warframe file list. Are you connected to the internet?");
            exit(1);
        }
    };
    let parsed = match exeupdate::parser::parse_file_list(index) {
        Ok(list) => list,
        Err(_) => {
            println!("Failed to parse Warframe file list. Did DE change something?");
            exit(1);
        }
    };
    let verbose = matches.is_present("verbose");
    for item in parsed {
        let check = match exeupdate::parser::categorize(&item) {
            Exe32Bit | LauncherAsset => true,
            SteamAsset => matches.is_present("steam"),
            GameAsset => matches.is_present("full"),
            Exe64Bit => matches.is_present("64bit"),
            Unknown => true
        } || matches.is_present("full");
        let display_path = item.disk_path.clone();
        if !check {
            if verbose {
                println!("SKIP {}", display_path);
            }
        } else {
            match exeupdate::checker::check_file(&item) {
                Ok(needs_update) => {
                    if needs_update {
                        if verbose {
                            println!("OUTD {}", display_path); //uptodate outofdate
                        } else {
                            println!("{}", display_path);
                        }
                    } else {
                        if verbose {
                            println!("OK   {}", display_path);
                        }
                    }
                },
                Err(err) => {
                    println!("ERR  {}", display_path);
                    println!("     {}", err);
                }
            }
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("parse") {
        // Create a path to the desired file
        let path = match matches.value_of("INPUT") {
            Some(p) => PathBuf::from(p),
            None => match paths::guess_log_folder_from_wineprefix() {
                Some(mut p) => {
                    p.push("Preprocess.log");
                    p
                },
                None => PathBuf::from("Preprocess.log")
            }
        };
        parse_file(path);
        return;
    } else if let Some(matches) = matches.subcommand_matches("wine-ver") {
        for (version, path) in wine::build_wine_versions_list() {
            if matches.is_present("paths") {
                println!("{}: {:?}", version, path);
            } else {
                println!("{}", version);
            }
        }
        return;
    }

    let wfpath = match paths::guess_game_install_dir_from_wineprefix() {
        Some(path) => path,
        None => {
            println!("Can't find Warframe! Is $WINEPREFIX set?");
            exit(1);
        }
    };

    if let Some(_) = matches.subcommand_matches("update") {
        update_game(wfpath);
    } else if let Some(_) = matches.subcommand_matches("run") {
        run_game(wfpath);
    } else if let Some(matches) = matches.subcommand_matches("checkupdate") {
        checkupdate(matches);
    }
}
