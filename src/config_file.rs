use crate::config::{Config, UsrConfig};
use serde::Deserialize;
use std::{env, fs, io, path::PathBuf};
use toml;

const CONFIG_FILE: &str = "dotfiles.toml";

#[derive(Deserialize, Debug)]
struct Dotfiles {
    configs: Vec<UsrConfig>,
}

fn read_config_file() -> Result<Vec<Config>, io::Error> {
    let config_path = env::current_dir()?.join(CONFIG_FILE);
    let config_content = fs::read_to_string(config_path).expect("Unable to read config file.");

    // TODO: is this thing smart enough to understand that I want something like
    // ```dotfiles.toml
    // [tmux]
    // target_path = "~/.config/tmux/"
    // is_recursive = false
    //
    // [nvim]
    // target_path = "~/.config/nvim/"
    // is_recursive = true
    // ```
    // to be turned into
    // ```
    // Dotfiles {
    //     configs: vec![
    //         UsrConfig {
    //             name: "tmux",
    //             options: vec![TargetPath("~/.config/tmux/"), IsRecursive(false)],
    //         },
    //         UsrConfig {
    //             name: "vim",
    //             options: vec![Targetpath("~/.config/nvim/"), IsRecursive(true)]
    //         },
    //     ]
    // }
    // ```
    let config_toml: Dotfiles = toml::from_str(&config_content).unwrap();

    let mut configs: Vec<Config> = Vec::new();
    for usr_conf in config_toml.configs.iter() {
        match Config::from_usr(usr_conf) {
            Ok(conf) => configs.push(conf),
            Err(message) => panic!("{}", message),
        }
    }
    Ok(configs)
}
