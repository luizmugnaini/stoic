use anyhow::bail;
use log::{debug, error, info, warn};
use path_absolutize::Absolutize;
use serde::Deserialize;
use std::{fs, io, os::unix, path::PathBuf};

#[derive(Deserialize, Debug)]
pub struct Node {
    /// The `src` of the node can be either a file or a directory.
    pub src: PathBuf,

    /// The `target` will always be a directory.
    pub target: PathBuf,

    /// If `src` is a directory, `recursive` determines whether we should traverse through its
    /// entries or bail.
    pub recursive: bool,
}

impl Node {
    fn solve_existing_link(&self, dest: &PathBuf) -> Result<(), io::Error> {
        debug!("Solving already existing symlink at destination {:?}", dest);
        match dest.as_path().read_link() {
            Ok(old_src) => {
                if old_src != self.src {
                    warn!(
                        "The dest {:?} is already a symlink, but it does not point to source {:?}\
                        \n[Previous symlink source: {:?}]",
                        dest, self.src, old_src
                    );
                    fs::remove_file(dest)?;
                    unix::fs::symlink(&self.src, dest)?;
                } else {
                    info!("Link at {:?} is already correctly set", old_src);
                }
            }
            Err(e) => {
                warn!("Removing broken symlink at {:?} due to error: {}", dest, e);
                fs::remove_file(dest)?;
                unix::fs::symlink(&self.src, dest)?;
            }
        }
        Ok(())
    }

    /// Handle the creation of a symlink for a given file object. The method will assume that
    /// `self.src` points to a file. We check if there already exists a symlink at `self.target`,
    /// if so, we check if it points to our current `self.src`. If positive, we skip the creation
    /// of a link. If negative, we overwrite this old link with the new source.
    fn make_file_link(self) -> Result<(), anyhow::Error> {
        match self.target.symlink_metadata() {
            Ok(meta) => {
                let ft = meta.file_type();
                if ft.is_symlink() {
                    self.solve_existing_link(&self.target)?;
                    return Ok(());
                } else {
                    // TODO: Add info on how to ignore the file in further runs when the feature is
                    // made available.
                    warn!(
                        "The destination path points to an already existing file or directory, skipping...\n\
                        \t-> target: {:?}\n\
                        If the user wants to replace that file by a symlink, they should \
                        deal with this conflict manually and run stoic again.",
                        self.target 
                    );
                    return Ok(());
                }
            }
            Err(_) => unix::fs::symlink(&self.src, &self.target)?,
        }
        info!(
            "Successful linking...\n\tsource: {:?}\n\tdestination: {:?}",
            self.src, self.target
        );
        Ok(())
    }

    fn handle_dir(self) -> Result<(), anyhow::Error> {
        if let Err(e) = fs::create_dir_all(&self.target) {
            bail!(
                "Unable to create target directory path {:?} due to error {:?}",
                self.target,
                e
            );
        }

        let entries = fs::read_dir(&self.src)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Vec<Result<PathBuf, _>>>();

        for e in entries {
            debug!("Checking directory entries of {:?}...", self.src);
            match e {
                Ok(src) => {
                    let src: PathBuf = match src.absolutize() {
                        Ok(cow_path) => cow_path.into(),
                        Err(err) => {
                            error!(
                                "Unable to absolutize path for {:?} of {:?} due to {:?}",
                                src, self.src, err
                            );
                            continue;
                        }
                    };

                    match (src.is_file(), src.is_dir(), src.is_symlink()) {
                        (true, _, _) => {
                            let target = match src.file_name() {
                                Some(e_fn) => self.target.join(e_fn),
                                None => {
                                    error!("Error reading file name of {:?}... skipping", src);
                                    continue;
                                }
                            };
                            let e = Node {
                                src,
                                target,
                                recursive: self.recursive,
                            };
                            if let Err(err) = e.make_file_link() {
                                error!("Unable to make link due to {:?}", err);
                            }
                        }
                        (_, true, _) => {
                            if self.recursive {
                                let target = match src.file_name() {
                                    Some(e_fn) => self.target.join(e_fn),
                                    None => {
                                        error!("Error reading file name of {:?}... skipping", src);
                                        continue;
                                    }
                                };
                                Node {
                                   src,
                                    target,
                                    recursive: self.recursive,
                                }
                                .make()?;
                            } else {
                                debug!("Recursive set to false, ignoring {:?}", src);
                            }
                        }
                        (_, _, true) => warn!("Source {:?} points to a symlink, skipping...", self.src),
                        _ => error!(
                            "Unable to determine {:?} file type: not a file, directory or symlink... skipping",
                            self.src
                        ),
                    }
                }
                Err(err) => {
                    error!(
                        "Error reading path for directory entry {:?} due to {:?}... skipping",
                        self.src, err
                    );
                }
            }
        }
        Ok(())
    }

    /// Given a node, handle the creation of all entries and subsequent recursive entries if
    /// specified.
    pub fn make(self) -> Result<(), anyhow::Error> {
        let src_ft = match self.src.symlink_metadata() {
            Ok(m) => m.file_type(),
            Err(e) => bail!(
                "Unable to retrive source {:?} metadata due to {:?}",
                self.src,
                e
            ),
        };
        match (src_ft.is_file(), src_ft.is_dir(), src_ft.is_symlink()) {
            (true, _, _) => {
                if let Err(e) = self.make_file_link() {
                    error!("Unable to make link due to {:?}", e);
                }
            }
            (_, true, _) => self.handle_dir()?,
            (_, _, true) => warn!("Source {:?} points to a symlink, skipping...", self.src),
            _ => bail!(
                "Unable to determine {:?} file type: not a file, directory or symlink",
                self.src
            ),
        }
        Ok(())
    }
}
