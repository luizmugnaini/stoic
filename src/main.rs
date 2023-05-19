mod config;
mod config_file;
use std::io;

fn main() -> Result<(), io::Error> {
    let dotfiles = config_file::read_dotfiles().expect("Unable to read dotfiles.toml");
    for node in dotfiles.get_nodes().iter() {
        node.create_symlink()?;
    }
    Ok(())
}
