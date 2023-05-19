use home;
use path_absolutize::Absolutize;
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
    pub fn new(source_path: String, mut target_path: String, is_recursive: Option<bool>) -> Self {
        if target_path[0..=1].eq("~/") {
            let home_dir = home::home_dir().unwrap();
            target_path = home_dir
                .join(Path::new(&target_path[2..]))
                .display()
                .to_string();
        } else {
            target_path = Path::new(&target_path)
                .absolutize()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
        }
        println!("target_path = {}", target_path);

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
        let target_path = Path::new(&target);
        match target_path.symlink_metadata() {
            // If the metadata exists, we should check if it is either a file or a symlink:
            // * If target isn't a symlink, we abort the creation and ask for the user to choose
            //   what to do with the file at `target` manually. This behaviour prevents the user
            //   from mindlessly losing data.
            // * If the target is indeed a symlink, simply delete the target and create another
            //   symlink. (TODO: this behaviour is clearly non-optimal, we could check if the
            //   symlink already points to `source` and only overwrite `target` if the intended
            //   `source` differs from the actual source of the symlink.)
            Ok(target_metadata) => {
                if target_metadata.file_type().is_symlink() {
                    fs::remove_file(target_path)?;
                    unix::fs::symlink(source, target)?;
                } else {
                    eprintln!(
                        "Aborted creation of symlink at {}, since the file already exists.",
                        target
                    );
                }
            }
            _ => {
                unix::fs::symlink(source, target)?;
            }
        }
        Ok(())
    }
}
