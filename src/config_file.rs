use crate::config::Config;
use serde::Deserialize;
use std::{env, fs, io};
use toml::Table;

const CONFIG_FILE: &str = "dotfiles.toml";

#[derive(Deserialize, Debug)]
pub struct Dotfiles {
    nodes: Vec<Config>,
}

impl Dotfiles {
    pub fn default() -> Self {
        Self { nodes: vec![] }
    }

    pub fn push(&mut self, node: Config) {
        self.nodes.push(node);
    }

    pub fn get_nodes(self) -> Vec<Config> {
        self.nodes
    }
}

pub fn read_dotfiles() -> Result<Dotfiles, io::Error> {
    let cwd = env::current_dir().expect("Unable to obtain the current working directory");
    let config_path = cwd.clone().join(CONFIG_FILE);
    let config_content = fs::read_to_string(config_path).expect("Unable to read config file.");
    let config_toml: Table = toml::from_str(&config_content).unwrap();

    let mut dotfiles = Dotfiles::default();
    for key in config_toml.keys() {
        // The `config_path` variable is optional, if the user omits it, the program
        // assumes that the config path relative to the `dotfiles.toml` file is
        // `"./key"`.
        let config_path = match config_toml[key].get("config_path") {
            Some(toml::Value::String(cp)) => cp,
            _ => key,
        };

        let target_path = match config_toml[key].get("target_path") {
            Some(toml::Value::String(tp)) => tp,
            _ => panic!("target_path not specified for {}", key),
        };

        let is_recursive = match config_toml[key].get("is_recursive") {
            Some(toml::Value::Boolean(b)) => Some(b).copied(),
            _ => None,
        };

        dotfiles.push(Config::new(
            cwd.join(config_path).display().to_string(),
            target_path.to_string(),
            is_recursive,
        ));
    }
    Ok(dotfiles)
}
