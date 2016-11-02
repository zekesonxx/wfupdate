use clap;
use super::super::{config, exeupdate, run, paths, logparser};
use std::process::{Stdio, exit};
use std::path::PathBuf;
use logparser::LogLine;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::create_dir_all;
use bytesize;
use bytesize::ByteSize;

pub fn subcommand<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(@subcommand update =>
        (about: "Updates the game")
        (@arg stage1: short("1") --stage1 "Run a stage 1 update (executables), defaults to both")
        (@arg stage2: short("2") --stage2 "Run a stage 2 update (game assets), defaults to both")
        (@arg verbose: -v --verbose "Output added debugging information")
        (@arg rawlines: -r --rawlines "Output raw log lines from Warframe")
        (@subcommand check =>
            (about: "Check for updates, but don't actually update")
            (@arg stage2: short("2") --stage2 "Check for a stage 2 update, only checks stage 1 by default")
            (@arg is64bit: short("6") long("64bit") "Include 64-bit files")
            (@arg fullcheck: -f --full "Check all the files, including the 35GB+ of game assets")
            (@arg silent: -s --silent "Don't output anything, just exit with a status code")
            (@arg verbose: -v --verbose "Output added debugging information")
        )
    )
}

pub fn run(matches: &clap::ArgMatches) {
    let config = config::get();
    let wfpath = match paths::game_install_dir() {
        Some(path) => path,
        None => {
            println!("Can't find Warframe! Is your wineprefix setup?");
            exit(1);
        }
    };

    if let Some(matches) = matches.subcommand_matches("check") {
        if matches.is_present("stage2") {
            println!("Running Stage 1 check...");
        }
        stage1_check(matches, &config);
        if matches.is_present("stage2") {
            println!("Running Stage 2 check...");
            if !stage2_check(matches, wfpath) {
                println!("No game assets need updating");
            }
        }
    } else {
        // To explain:
        // flags | stage1 | stage2
        //       |  true  |  true
        //  -1   |  true  | false
        //  -2   |  false |  true
        //  -12  |  true  |  true

        let both = !(matches.is_present("stage1") || matches.is_present("stage2"));
        let stage1 = matches.is_present("stage1") || both;
        let stage2 = matches.is_present("stage2") || both;
        let both = stage1 && stage2;

        if stage1 {
            if both {
                println!("Running Stage 1 update...");
            }
            stage1_update(&config);
        }
        if stage2 {
            if both {
                println!("Running Stage 2 update...");
            }
            stage2_update(matches, wfpath);
        }
    }
}


fn stage1_check(matches: &clap::ArgMatches, config: &::ini::Ini) {
    use exeupdate::FileType::*;
    let verbose = matches.is_present("verbose");
    let silent = matches.is_present("silent");
    if !silent { println!("Getting file list..."); }
    let index = match exeupdate::downloader::get_index() {
        Ok(index) => index,
        Err(_) => {
            println!("Failed to get Warframe file list. Are you connected to the internet?");
            exit(1);
        }
    };
    if !silent { println!("Parsing file list..."); }
    let parsed = match exeupdate::parser::parse_file_list(index) {
        Ok(list) => list,
        Err(_) => {
            println!("Failed to parse Warframe file list. Did DE change something?");
            exit(1);
        }
    };
    if !silent { println!("Checking Files..."); }
    for item in parsed {
        let check = match exeupdate::parser::categorize(&item) {
            Exe32Bit | LauncherAsset => true,
            SteamAsset => config::parse_bool(config.get_from(Some("update"), "steam")),
            GameAsset => matches.is_present("full"),
            Exe64Bit => matches.is_present("64bit") || config::parse_bool(config.get_from(Some("game"), "64bit")),
            Unknown => true
        } || matches.is_present("full");
        let display_path = item.disk_path.clone();
        if !check {
            if verbose && exeupdate::parser::categorize(&item) != GameAsset {
                println!("Skipping {}", display_path);
            }
        } else {
            match exeupdate::checker::check_file(&item) {
                Ok(needs_update) => {
                    if needs_update {
                        if verbose {
                            println!("Needs update: {}", display_path);
                        } else if !silent {
                            println!("{}", display_path);
                        }
                    } else {
                        if verbose {
                            println!("Up to date: {}", display_path);
                        }
                    }
                },
                Err(err) => {
                    println!("Error {}", display_path);
                    println!("{}", err);
                }
            }
        }
    }
}

fn stage2_check(matches: &clap::ArgMatches, wfpath: PathBuf) -> bool {
    let mut program = match run::build_game_update(wfpath)
    .stdout(Stdio::piped())
    .stderr(Stdio::null())
    .spawn() {
        Ok(child) => child,
        Err(_) => {
            println!("Cannot run Warframe to check for updates");
            exit(1);
        },
    };
    let mut files = 0usize;
    match program.stdout.take().as_mut() {
        Some(out) => {
            let buf_reader = BufReader::new(out);
            for line in buf_reader.lines() {
                match line {
                    Ok(l) => {
                        let parsedline = logparser::parse_line(l.as_str());
                        if let LogLine::BytesToDownload(bytes) = parsedline {
                            let _ = program.kill();
                            if !matches.is_present("silent") {
                                println!("{} file{}, {} bytes", files,
                                         if files != 1 {"s"} else {""}, bytes);
                            }
                            return true;
                        } else if let LogLine::HashMismatch(_) = parsedline {
                            files += 1;
                        }
                    },
                    Err(_) => return false,
                };
            }
        },
        None => return false,
    }
    return false;
}


fn stage1_update(config: &::ini::Ini) {
    use exeupdate::FileType::*;
    println!("Downloading file list...");
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
    let mut to_update: Vec<exeupdate::File> = vec![];
    println!("Checking files...");
    for item in parsed {
        let check = match exeupdate::parser::categorize(&item) {
            Exe32Bit | LauncherAsset => true,
            SteamAsset => config::parse_bool(config.get_from(Some("update"), "steam")),
            GameAsset => false,
            Exe64Bit => config::parse_bool(config.get_from(Some("game"), "64bit")),
            Unknown => true
        };
        let display_path = item.disk_path.clone();
        if check {
            match exeupdate::checker::check_file(&item) {
                Ok(needs_update) => {
                    if needs_update {
                        to_update.push(item);
                    }
                },
                Err(err) => {
                    println!("Failed to check {}", display_path);
                    println!("{}", err);
                }
            }
        }
    }
    println!("{} file{} to update", to_update.len(), if to_update.len() != 1 {"s"} else {""});
    for file in to_update {
        let display = file.disk_path.clone();
        println!("Downloading {} ({})", display, bytesize::ByteSize::b(file.size as usize));
        let newcontent = match exeupdate::downloader::get_file(file.download_path) {
            Ok(content) => content,
            Err(err) => {
                println!("Failed to download {}", display);
                println!("{:?}", err);
                exit(1);
            }
        };
        println!("Applying updated file");
        let disk_path = paths::realize_path(file.disk_path).unwrap();
        create_dir_all(disk_path.parent().unwrap()).unwrap();
        match exeupdate::update::update_file(disk_path, newcontent) {
            Ok(_) => {
                println!("File successfully updated");
            },
            Err(err) => {
                println!("Failed to update {}", display);
                println!("{:?}", err);
                exit(1);
            }
        };
    }
}

fn stage2_update(matches: &clap::ArgMatches, wfpath: PathBuf) {
    let mut parsed: Vec<LogLine> = vec![];
    let mut program = match run::build_game_update(wfpath)
    .stdout(Stdio::piped())
    .stderr(Stdio::null())
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
            for line in buf_reader.lines() {
                match line {
                    Ok(l) => {
                        if matches.is_present("rawlines") {
                            println!("{}", l);
                        }
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
                            if matches.is_present("rawlines") {
                                println!("{}", display_parsed(&parsed));
                            } else {
                                print!("\x1b[0K\r{}", display_parsed(&parsed));
                            }
                        }
                    },
                    Err(_) => return,
                };
            }
        },
        None => return,
    }
}


// TODO: Fix display_parsed and percentage. They're shit.
fn display_parsed(parsed: &Vec<LogLine>) -> String {
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
    format!("{}; {}", bytes, filecount)
}

fn percentage(amount: u64, total: u64) -> String {
    if total == 0 {
        return String::from("");
    }
    let frac: f64 = (amount*100u64) as f64/(total*100u64) as f64;

    let mut output = format!("{}", frac*100f64);
    output.truncate(5);
    output
}
