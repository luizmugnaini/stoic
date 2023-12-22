mod config;
mod node;

use clap::{value_parser, Arg, Command};
use config::ConfigFile;

use log::{error, LevelFilter};
use std::{io, process};

fn main() -> Result<(), io::Error> {
    let matches = Command::new("stoic")
        .arg(
            Arg::new("log")
                .short('l')
                .long("log")
                .default_value("info")
                .value_name("LOG LEVEL")
                .help("Log level for stoic.")
                .value_parser(value_parser!(LevelFilter)),
        )
        .get_matches();
    env_logger::Builder::default()
        .filter_level(*matches.get_one("log").unwrap())
        .format_timestamp(None)
        .init();
    let config = match ConfigFile::read_config() {
        Ok(c) => c,
        Err(err) => {
            error!("Unable to read configuration file due to: {:?}", err);
            process::exit(1);
        }
    };

    config.make_links();
    Ok(())
}
