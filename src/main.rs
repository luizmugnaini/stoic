mod config;
mod node;

use config::ConfigFile;

use log::error;
use log::LevelFilter;
use std::env;
use std::io;
use std::process;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let flvl = if args.len() > 1 {
        match args[1].as_str() {
            "-d" | "--debug" => Some(LevelFilter::Debug),
            "-s" | "--silent" => Some(LevelFilter::Error),
            "--no-msg" => Some(LevelFilter::Off),
            _ => Some(LevelFilter::Info),
        }
    } else {
        None
    };
    env_logger::Builder::default()
        .filter_level(flvl.unwrap_or(LevelFilter::Off))
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
