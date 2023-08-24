use crate::node::Node;
use home;
use log::{error, info};
use path_absolutize::Absolutize;
use serde::Deserialize;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};
use toml::Table;

const CONFIG_FILE: &str = "stoic.toml";

#[derive(Deserialize, Debug)]
pub struct ConfigFile {
    pub root: PathBuf,
    pub nodes: Vec<Node>,
}

fn find_config(mut cwd: PathBuf) -> Result<PathBuf, Box<io::Error>> {
    let config_path = cwd.join(CONFIG_FILE);
    if let true = config_path.as_path().exists() {
        return Ok(config_path);
    }

    // If the path still has parents, continue the search.
    if cwd.pop() {
        return find_config(cwd);
    }
    return Err(Box::new(io::Error::new(
        io::ErrorKind::NotFound,
        "Config file could not be found",
    )));
}

pub fn read_config<'a>() -> Result<ConfigFile, io::Error> {
    let mut config_path = match find_config(env::current_dir()?) {
        Ok(p) => p,
        Err(b) => return Err(*b),
    };
    info!("Reading config file at {:?}", config_path);
    let config_content = match fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(_) => match fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(e_alt) => return Err(e_alt),
        },
    };
    // Remove "stoic.toml" from the path for later use.
    config_path.pop();

    let config: Table = toml::from_str(&config_content).unwrap();

    let mut dotfiles = ConfigFile {
        root: config_path.clone(),
        nodes: vec![],
    };
    for key in config.keys() {
        let src = match config[key].get("src") {
            Some(toml::Value::String(src)) => PathBuf::from(src),
            _ => PathBuf::from(key),
        };
        let target = match config[key].get("target") {
            Some(toml::Value::String(tp)) => {
                if tp[0..=1].eq("~/") {
                    let home_dir = home::home_dir().unwrap();
                    home_dir.join(Path::new(&tp[2..]))
                } else {
                    config_path
                        .join(Path::new(&tp))
                        .absolutize()
                        .unwrap()
                        .into()
                }
            }
            _ => {
                error!("Target not specified for {:?}, skipping...", key);
                continue;
            }
        };
        let recursive = match config[key].get("recursive") {
            Some(toml::Value::Boolean(b)) => Some(b).copied(),
            _ => None,
        };

        dotfiles.nodes.push(Node {
            src,
            target,
            recursive,
        });
    }
    Ok(dotfiles)
}
