use crate::node::Node;
use anyhow::anyhow;
use log::{error, info};
use path_absolutize::Absolutize;
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use toml::Table;

const CONFIG_FILE: &str = "stoic.toml";

#[derive(Deserialize, Debug)]
pub struct ConfigFile {
    nodes: Vec<Node>,
}

impl ConfigFile {
    fn cfg_dir(mut cwd: PathBuf) -> Result<PathBuf, anyhow::Error> {
        let config_path = cwd.join(CONFIG_FILE);
        if let true = config_path.as_path().exists() {
            return Ok(cwd);
        }

        // If the path still has parents, continue the search.
        if cwd.pop() {
            return ConfigFile::cfg_dir(cwd);
        }
        Err(anyhow!(
            "Config file {:?} couldn't be found...",
            CONFIG_FILE
        ))
    }

    pub fn read_config() -> Result<ConfigFile, anyhow::Error> {
        let config_dir = ConfigFile::cfg_dir(env::current_dir()?)?;
        let config_path = config_dir.join(CONFIG_FILE);
        let content = fs::read_to_string(&config_path)?;
        let toml: Table = toml::from_str(&content)?;
        info!("Reading config file at {:?}", config_path);

        let mut config = ConfigFile { nodes: vec![] };
        for key in toml.keys() {
            if key.as_str() == "stoic" {
                continue;
            }
            let src = match toml[key].get("src") {
                Some(toml::Value::String(src)) => {
                    config_dir.join(Path::new(src)).absolutize().unwrap().into()
                }
                _ => config_dir.join(Path::new(key)).absolutize().unwrap().into(),
            };
            let target = match toml[key].get("target") {
                Some(toml::Value::String(tp)) => match &tp[0..=1] {
                    "~/" => home::home_dir().unwrap().join(Path::new(&tp[2..])),
                    _ => config_dir.join(Path::new(&tp)).absolutize().unwrap().into(),
                },
                _ => {
                    error!("Target not specified for {:?}, skipping...\n\
                        [The user should make sure to specify the target directory and run stoic again]", key);
                    continue;
                }
            };
            let recursive = match toml[key].get("recursive") {
                Some(toml::Value::Boolean(b)) => Some(*b).unwrap_or(false),
                _ => false,
            };

            config.nodes.push(Node {
                src,
                target,
                recursive,
            });
        }
        Ok(config)
    }

    pub fn make_links(self) {
        self.nodes.into_iter().for_each(|n| {
            if let Err(e) = n.make() {
                error!("Received an error while processing node: {:?}", e);
            }
        });
    }
}
