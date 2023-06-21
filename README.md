# stoic-dotfiles 🖇

<p align="center">
  <img src="stoic.png" alt="Random stoic bust" width="150"/>
</p>


A helper for managing dotfiles (alternative to [`stow`](https://www.gnu.org/software/stow/)).

## Use case example

Assume Bob has a bunch of dotfiles he wishes to safely store in the cloud in
order to be able to move his configs between machines. He then creates a
directory `my-configs` and for each _program_ he has a directory of the form
`my-configs/program` where he may store its configs. In order to use
`stoic-dotfiles` he first installs it from [crates.io](https://crates.io) via
```shell
cargo install stoic-dotfiles
```
In order to correctly use `stoic-dotfiles`, Bob creates a file
`my-configs/dotfiles.toml` where and each program can be configured using two
variables:
* `target_path`: a string containing the path where the symlinks should be
  created. Such path can be either relative or absolute (the program also
  resolves paths starting with `"~/"`).

  For instance, if Bob has a file `~/my-configs/nvim/init.vim` and has
  ```toml
  [nvim]
  target_path = "~/.config/nvim"
  ```
  in his `dotfiles.toml` file, then after running
  ```shell
  cd ~/my-configs
  stoic-dotfiles
  ```
  the program creates a symlink
  ```
  ~/.config/nvim/init.lua -> ~/my-configs/nvim/init.lua
  ```
* `is_recursive` (optional): whether or not the program should create symlinks for
  subdirectories present in `my-configs/program`. If the variable isn't set, the
  program will assume that `is_recursive = false`.

  Suppose that Bob wants to recursively link his `nvim` configs and has a file
  `~/my-configs/nvim/plugins/lsp.lua` and adds to his `dotfiles.toml` the
  following content:
  ```toml
  [nvim]
  target_path = "~/.config/nvim"
  is_recursive = true
  ```
  Then after running `stoic-dotfiles` inside `~/my-configs` the program should
  create symlinks:
  ```
  ~/.config/nvim/init.lua -> ~/my-configs/nvim/init.lua
  ~/.config/nvim/plugins/lsp.lua -> ~/my-configs/nvim/plugins/lsp.lua
  ```
* `config_path` (optional): string containing the path to the configuration
  directory to be the source of the symlinks, if the variable isn't set, the
  program will assume that the relative path to `dotfile.toml` is `"./key"` for
  the corresponding `[key]` in the config file.

  For instance suppose Bob wants to store all his tmux-related configurations
  in a single directory but does not want all files to go be symlinked to the
  same relative target directory. He can obtain this by adding the following to his `dotfiles.toml`:
  ```toml
  [tmux]
  target_path = "~/.config/tmux"

  [tmux_scripts]
  config_path = "tmux/scripts"
  target_path = "~/.local/bin"
  ```
