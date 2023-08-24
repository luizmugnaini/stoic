use log::{debug, error, info, warn};
use path_absolutize::Absolutize;
use serde::Deserialize;
use std::{fs, io, os::unix, path::PathBuf};

#[derive(Deserialize, Debug)]
pub struct Node {
    pub src: PathBuf,
    pub target: PathBuf,
    pub recursive: Option<bool>,
}

impl Node {
    pub fn make_symlinks(&self) -> Result<(), io::Error> {
        debug!("Making symlinks for {:?}", self);
        fs::create_dir_all(&self.target)?;
        if self.src.is_file() {
            debug!("The src {:?} is a file... making file symlink", self.src);
            match Node::file_symlink(&self.src, &self.target) {
                Ok(()) => info!(
                    "Successful linking...\nsource: {:?}\ntarget: {:?}",
                    self.src, self.target
                ),
                Err(e) => warn!("Unable to create symlink due to {:?}", e),
            }
        } else if self.src.is_dir() && self.recursive.unwrap_or(false) {
            debug!("The src {:?} is a directory... recursing", self.src);
            let entries = fs::read_dir(&self.src)?
                .map(|res| res.map(|e| e.path()))
                .collect::<Vec<Result<PathBuf, _>>>();

            for e in entries {
                debug!("Checking directory entries of {:?}...", self.src);
                match e {
                    Ok(e_src) => {
                        debug!("{:?} is a valid entry", e_src);
                        let e_src: PathBuf = e_src.absolutize().unwrap().into();
                        let e_target = match e_src.file_name() {
                            Some(e_fn) => self.target.join(e_fn),
                            None => {
                                error!("Error reading file name of {:?}... skipping", e_src);
                                continue;
                            }
                        };
                        Node::make_symlinks(&Node {
                            src: e_src,
                            target: e_target,
                            recursive: self.recursive,
                        })?;
                    }
                    Err(err) => {
                        error!(
                            "Error reading path for directory entry {:?} due to {:?}",
                            self.src, err
                        );
                    }
                }
            }
        }
        Ok(())
    }

    fn solve_target_symlink(src: &PathBuf, target: &PathBuf) -> Result<(), io::Error> {
        debug!("Solving already existing symlink at target");
        // BUG: This is always returning an error for "invalid argument"??
        match target.as_path().read_link() {
            Ok(old_src) => {
                if old_src != *src {
                    warn!(
                        "The target {:?} is already a symlink, but it does not point to source {:?}\
                        \n[Previous symlink source: {:?}]",
                        target, src, old_src
                    );
                    fs::remove_file(target)?;
                    unix::fs::symlink(src, target)?;
                } else {
                    info!("Link at {:?} is already correctly set", old_src);
                }
            }
            Err(e) => {
                warn!(
                    "Removing broken symlink at {:?} due to error: {}",
                    target, e
                );
                fs::remove_file(target)?;
                unix::fs::symlink(src, target)?;
            }
        }
        Ok(())
    }

    fn file_symlink(src: &PathBuf, target: &PathBuf) -> Result<(), io::Error> {
        match target.symlink_metadata() {
            Ok(meta) => {
                if meta.file_type().is_file() {
                    warn!(
                        "The target points to an already existing file...\n\
                        -> target: {:?}\n\
                        If the user wants to replace that file by a symlink, the user should\
                        deal with this conflict manually and run stoic again",
                        target
                    );
                } else {
                    debug!(
                        "The target {:?} exists but isn't a file. Proceeding to make symlink...\n",
                        target
                    );
                    Node::solve_target_symlink(src, target)?;
                }
            }
            Err(e_meta) => {
                debug!(
                    "Unable to get metadata for {:?} due to {:?}\
                    \nProceeding to make symlink...",
                    target, e_meta
                );
                if let Err(e) = unix::fs::symlink(src, target) {
                    warn!(
                        "Unsuccessful linking of {:?} to {:?} due to {:?}",
                        target, src, e
                    );
                }
            }
        }
        Ok(())
    }
}
