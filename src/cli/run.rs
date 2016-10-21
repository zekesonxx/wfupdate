use clap;
use super::super::{run, paths};
use std::process::{Stdio, exit};
use std::os::unix::process::CommandExt; //so we can call .exec() on a Command which invokes execvp(3).

pub fn subcommand<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(@subcommand run =>
        (about: "Launches Warframe")
        (@arg launcher:  --launcher "Start the official launcher instead of running the game directly")
        (@arg mumble: -m --mumble "Include the Mumble overlay (not implemented)")
        (@arg silent: -s --silent "Don't output Warframe's stdout/stderr")
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
    let mut program = if matches.is_present("launcher") {
        let launcherpath = paths::launcher_exe().unwrap();
        run::launcher_executable(launcherpath)
    } else {
        run::build_game_run(wfpath)
    };
    if matches.is_present("silent") {
        program.stdout(Stdio::null());
        program.stderr(Stdio::null());
    }
    program.exec();
    panic!("Couldn't run Warframe");
}
