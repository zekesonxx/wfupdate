use clap;
use super::super::{run, paths};
use std::process::exit;
use std::os::unix::process::CommandExt;

pub fn subcommand<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(@subcommand repair =>
        (about: "Runs Warframe's repair function")
        (@arg optimize: -o --optimize alias("defrag") "Instead of repairing, run the defragger (the \"Optimize\" button in the official launcher).")
    )
}

pub fn run(matches: &clap::ArgMatches) {
    let wfpath = match paths::game_install_dir() {
        Some(path) => path,
        None => {
            println!("Can't find Warframe! Is your wineprefix setup?");
            exit(1);
        }
    };
    let mut program = if matches.is_present("optimize") {
        run::build_game_defrag(wfpath)
    } else {
        run::build_game_repair(wfpath)
    };

    program.exec();
    panic!("Couldn't run Warframe");
}
