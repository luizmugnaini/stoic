use crate::config::Config;
use serde::Deserialize;
use std::{env, fs, io};
use toml::Table;

const CONFIG_FILE: &str = "dotfiles.toml";

#[derive(Deserialize, Debug)]
pub struct Dotfiles {
    pub configs: Vec<Config>,
}

impl Dotfiles {
    pub fn default() -> Self {
        Self { configs: vec![] }
    }

    pub fn push(&mut self, node: Config) {
        self.configs.push(node);
    }

    pub fn get_nodes(self) -> Vec<Config> {
        self.configs
    }
}

pub fn read_dotfiles() -> Result<Dotfiles, io::Error> {
    let cwd = env::current_dir().expect("Unable to obtain the current working directory");
    let dotfiles_config_path = cwd.clone().join(CONFIG_FILE);
    let dotfiles_config_content =
        fs::read_to_string(dotfiles_config_path).expect("Unable to read config file.");
    let dotfiles_config_toml: Table = toml::from_str(&dotfiles_config_content).unwrap();

    let mut dotfiles = Dotfiles::default();
    for key in dotfiles_config_toml.keys() {
        let target_path = match dotfiles_config_toml[key].get("target_path") {
            Some(toml::Value::String(tp)) => tp.to_owned(),
            // fs::canonicalize(tp.to_owned())
            // .expect(
            //     "Unable to
            // obtain absolute path out of target_path.",
            // )
            // .to_str()
            // .unwrap(),
            _ => panic!("target_path not specified for {}", key),
        };

        let is_recursive = match dotfiles_config_toml[key].get("is_recursive") {
            Some(toml::Value::Boolean(b)) => Some(b).copied(),
            _ => None,
        };

        dotfiles.push(Config::new(
            cwd.join(key).display().to_string(),
            target_path.to_string(),
            is_recursive,
        ));
    }
    Ok(dotfiles)
}
