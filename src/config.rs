use serde::Deserialize;
use std::{env, fs, io, os::unix, path::PathBuf};

#[derive(Deserialize, Debug)]
pub struct UsrConfig {
    pub name: String,
    pub options: Vec<ConfigOptions>,
}

#[derive(Deserialize, Debug)]
pub enum ConfigOptions {
    TargetPath(String),
    IsRecursive(bool),
}

#[derive(Deserialize, Debug)]
pub struct Config {
    source_path: String,
    target_path: String,
    is_recursive: Option<bool>,
}

impl Config {
    fn new(source_path: String, target_path: String, is_recursive: Option<bool>) -> Self {
        Self {
            source_path,
            target_path,
            is_recursive,
        }
    }

    pub fn from_usr(usr_config: &UsrConfig) -> Result<Self, String> {
        let mut source_path = env::current_dir()
            .expect("Unable to obtain the current directory path.")
            .to_str()
            .unwrap()
            .to_string();
        source_path.push_str(&usr_config.name);
        let mut target_path = String::new();
        let mut is_recursive = None;

        for option in &usr_config.options {
            match option {
                ConfigOptions::TargetPath(path) => target_path.push_str(path),
                ConfigOptions::IsRecursive(opt) => is_recursive = Some(opt).copied(),
            }
        }

        if target_path.is_empty() {
            let message = format!("Target path is missing for {}", usr_config.name);
            return Err(message);
        }

        Ok(Self {
            source_path,
            target_path,
            is_recursive,
        })
    }

    fn create_simlink(self) -> Result<(), io::Error> {
        let entries = fs::read_dir(self.source_path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()?;

        for entry in entries {
            let entry_target = PathBuf::from(&self.target_path).join(entry.file_name().unwrap());

            // If `entry` is a directory and the recursive option is set to `true`, then we
            // should also create simlinks for the files contained in `entry`; otherwise we
            // simply skip the entry.
            if entry.is_dir() {
                match self.is_recursive {
                    Some(true) => {
                        let entry_config = Config::new(
                            entry.display().to_string(),
                            entry_target.display().to_string(),
                            self.is_recursive,
                        );
                        entry_config.create_simlink()?;
                    }
                    _ => continue,
                }
            } else {
                unix::fs::symlink(entry, entry_target)?;
            }
        }
        Ok(())
    }
}
