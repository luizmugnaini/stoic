mod config;
mod node;

use env_logger;
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
    let dotfiles = match config::read_config() {
        Ok(df) => df,
        Err(err) => {
            error!("Unable to read configuration file due to: {:?}", err);
            process::exit(1);
        }
    };

    for node in dotfiles.nodes.iter() {
        if let Err(e) = node.make_symlinks() {
            error!("Unable to handle {:?} due to error {:?}", node, e);
        }
    }
    Ok(())
}
