use clap;
use super::super::{config, wine, run};
use std::path::PathBuf;
use std::os::unix::process::CommandExt;

pub fn subcommand<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(@subcommand wine =>
        (about: "Wine-specific commands")
        (@setting SubcommandRequiredElseHelp)
        (@subcommand list =>
            (about: "List available Wine versions")
            (@arg paths: -p --paths "Display the detected paths")
        )
        (@subcommand set =>
            (about: "Sets the wine version")
            (@arg version: +required "The version to use, as specified by list.")
        )
        (@subcommand envvar =>
            (about: "Debug tool for Wine env vars, outputs a shell script")
            (@arg unset: -u --unset "Output a script to undo the effects of running envvar")
        )
        (@subcommand winecfg =>
            (about: "Runs winecfg")
        )
    )
}

pub fn run(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        ("list", Some(matches)) => {
            for version in wine::build_wine_versions_list() {
                if matches.is_present("paths") {
                    println!("{}: {:?} {:?}", version.version, version.path, version.source);
                } else {
                    println!("{}", version.version);
                }
            }
        },
        ("set", Some(matches)) => {
            let requested: String = matches.value_of("version").unwrap().to_lowercase();
            for version in wine::build_wine_versions_list() {
                if requested == version.version.to_lowercase() {
                    let mut config = config::get();
                    let mut winebin = version.path.clone();
                    winebin.push("bin");
                    let mut winelib = version.path.clone();
                    winelib.push("lib");
                    config.set_to(Some("wine"), "winebin".to_string(), winebin.as_os_str().to_os_string().into_string().unwrap());
                    config.set_to(Some("wine"), "winelib".to_string(), winelib.as_os_str().to_os_string().into_string().unwrap());
                    config::set(config);
                    return;
                }
            }
            println!("Couldn't find version \"{}\", does it show up in `wfupdate wine list`?", matches.value_of("version").unwrap());
        },
        ("winecfg", _) => {
            run::wine_cmd().arg("winecfg").exec();
        },
        ("envvar", Some(matches)) => {
            let config = config::get();
            if matches.is_present("unset") {
                println!("# Use by running: eval \"$(wfupdate wine envvar -u)\"");
                if let Some(_) = config.get_from(Some("wine"), "wineprefix") {
                    println!("unset WINEPREFIX");
                }
                if let Some(_) = config.get_from(Some("wine"), "winearch") {
                    println!("unset WINEARCH");
                }
                if let Some(_) = config.get_from(Some("wine"), "winebin") {
                    println!("export PATH=\"$WFUPDATE_BACKUP_PATH\"");
                    println!("unset WFUPDATE_BACKUP_PATH");
                    println!("unset WINE");
                }
                if let Some(_) = config.get_from(Some("wine"), "winelib") {
                    println!("export LD_LIBRARY_PATH=\"$WFUPDATE_BACKUP_LD_LIBRARY_PATH\"");
                    println!("unset WFUPDATE_BACKUP_LD_LIBRARY_PATH");
                }
            } else {
                println!("# Use by running: eval \"$(wfupdate wine envvar)\"");
                if let Some(value) = config.get_from(Some("wine"), "wineprefix") {
                    println!("export WINEPREFIX='{}'", value);
                }
                if let Some(value) = config.get_from(Some("wine"), "winearch") {
                    println!("export WINEARCH='{}'", value);
                }
                if let Some(value) = config.get_from(Some("wine"), "winebin") {
                    println!("export WFUPDATE_BACKUP_PATH=\"$PATH\"");
                    println!("export PATH=\"{}:$PATH\"", value);
                    let mut winebinary = PathBuf::from(value);
                    winebinary.push("wine");
                    println!("export WINE={:?}", winebinary); //The PathBuf debug display includes quotes
                }
                if let Some(value) = config.get_from(Some("wine"), "winelib") {
                    println!("export WFUPDATE_BACKUP_LD_LIBRARY_PATH=\"$LD_LIBRARY_PATH\"");
                    println!("export LD_LIBRARY_PATH=\"{}:$LD_LIBRARY_PATH\"", value);
                }
            }
        },
        _ => unreachable!()
    }
}
