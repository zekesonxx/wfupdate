use clap;
use super::super::config;
use std::process::exit;

pub fn subcommand<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(@subcommand config =>
        (about: "Gets and sets config variables")
        (@setting SubcommandRequiredElseHelp)
        (@subcommand get =>
            (about: "Gets a config variable")
            (display_order: 1000)
            (@arg key: +required "Key to get variable of")
        )
        (@subcommand set =>
            (about: "Sets a config variable")
            (display_order: 1001)
            (@arg key: +required "Key to get variable of")
            (@arg value: +required "Value to set the key to")
            (@arg quiet: -q --quiet "Don't output a confirmation message")
        )
    )
}

pub fn run(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        ("get", Some(matches)) => {
            let parsed = config::parse_configid(matches.value_of("key").unwrap());
            let config = config::get();
            match config.get_from(parsed.0, parsed.1.as_str()) {
                Some(k) => println!("{}", k),
                None => exit(2)
            }
        },
        ("set", Some(matches)) => {
            let parsed = config::parse_configid(matches.value_of("key").unwrap());
            let mut config = config::get();
            config.set_to(parsed.0, parsed.1, matches.value_of("value").unwrap().to_string());
            config::set(config);
            if !matches.is_present("quiet") {
                println!("Successfully set config variable {}.", matches.value_of("key").unwrap());
            }
        },
        _ => unreachable!()
    }
}
