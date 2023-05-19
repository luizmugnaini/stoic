use serde::Deserialize;
use std::{
    fs, io,
    os::unix,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug)]
pub struct Config {
    source_path: String,
    target_path: String,
    is_recursive: Option<bool>,
}

impl Config {
    pub fn new(source_path: String, target_path: String, is_recursive: Option<bool>) -> Self {
        Self {
            source_path,
            target_path,
            is_recursive,
        }
    }

    pub fn create_symlink(&self) -> Result<(), io::Error> {
        // TODO: What if `source_path` is not a directory but a file: we should also be
        // able to create the symlink.

        // Crate the target directory if it does not already exist (along with all
        // parent directories).
        fs::create_dir_all(&self.target_path).expect(&format!(
            "Unable to create directory path {}",
            self.target_path
        ));

        if Path::new(&self.source_path).is_dir() {
            // Collect all file paths contained in the `source_path` directory.
            let entries = fs::read_dir(&self.source_path)
                .expect(&format!(
                    "Unable to read the contents of {}",
                    &self.source_path
                ))
                .map(|res| {
                    res.map(|e| e.path())
                        .expect("Unable to obtain path out of entry.")
                })
                .collect::<Vec<PathBuf>>();

            // For each of `entry` contained in `source_path`:
            // * Check if `entry` is a directory. If positive and `is_recursive` is set to
            //   true, then we should create a `Config` for this `entry` and recursively
            //   call `create_symlink`.
            // * If `entry` isn't a directory but solely a file, create the symlink at the
            //   required target.
            for entry in entries {
                let entry_target =
                    PathBuf::from(&self.target_path).join(entry.file_name().unwrap());

                // If `entry` is a directory and the recursive option is set to `true`, then we
                // should also create symlinks for the files contained in `entry`; otherwise we
                // simply skip the entry.
                if entry.is_dir() {
                    match self.is_recursive {
                        Some(true) => {
                            let entry_config = Config::new(
                                entry.display().to_string(),
                                entry_target.display().to_string(),
                                self.is_recursive,
                            );
                            entry_config
                                .create_symlink()
                                .expect("Unable to recursively create symlink.");
                        }
                        _ => continue,
                    }
                } else {
                    Config::single_file_symlink(
                        &entry.display().to_string(),
                        &entry_target.display().to_string(),
                    )
                    .expect("Unable to create symlink.");
                }
            }
        } else {
            Config::single_file_symlink(&self.source_path, &self.target_path)
                .expect("Unable to create symlink.");
        }
        Ok(())
    }

    fn single_file_symlink(source: &str, target: &str) -> Result<(), io::Error> {
        // Precedure:
        // * Check if `target` exists. If it is a file, abort creation of the symlink,
        //   comunicate to the user.
        // * Check if the symlink already exists, if it does and points to `source`, we
        //   skip its creation.
        // * If the symlink exists but does not point to `source`, delete the symlink
        //   and create a new one pointing to `source`.
        let target_path = Path::new(&target);
        let target_metadata = target_path.symlink_metadata()?;
        // TODO: if the symlink points to a non-existing source, the `exists` method
        // will return `false`, we should be able to deal with this broken symlink,
        // remove it and create a correct one.
        if target_path.exists() {
            if target_metadata.file_type().is_symlink() {
                let target_symlink_source = target_path
                    .read_link()
                    .expect("Unable to follow link from target existing symlink.");
                eprintln!("{:?}", target_symlink_source.display());
                if PathBuf::from(source) != target_symlink_source {
                    fs::remove_file(target_path)?;
                    unix::fs::symlink(source, target)?;
                }
            } else {
                eprintln!("Aborted creation of symlink at {}, since the file already exists. You should manually delete or move it somewhere else in order for me to create the symlink.", target);
            }
        } else {
            eprintln!("im here");
            unix::fs::symlink(source, target)?;
        }
        Ok(())
    }
}
