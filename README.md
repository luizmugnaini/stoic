# stoic 🖇

<p align="center">
  <img src="stoic.png" alt="Random stoic bust" width="150"/>
</p>

Stoic is a CLI tool built using Rust. It aims to simplify the management of
configuration files (aka. dotfiles), in a centralized manner.
With Stoic, you can manage your dotfiles and maintain consistency across
different environments.

## Use case example

Assume Bob has a bunch of dotfiles he wishes to safely store in the cloud in
order to be able to move his configs between machines. He then creates a
directory `my-configs` and for each _program_ he has a directory of the form
`my-configs/program` where he may store its configs. In order to use
`stoic-dotfiles` he first installs it from [crates.io](https://crates.io) via

```shell
cargo install stoic-dotfiles
```

The binary will be aliased to `stoic`.

In order to correctly use `stoic`, Bob creates puts his configs in a directory called `my-configs` which has the following structure:

```
my-configs
├── nvim
│  └── init.lua
├── tmux-bob
│  ├── scripts
│  │  └── sessions.sh
│  └── tmux.conf
└── stoic.toml
```

the file `stoic.toml` is where and each program can be configured using three variables:

- `target`: a string containing the path where the symlinks should be
  created at. Such path can be either relative or absolute (the program also
  resolves paths starting with `"~/"`).

  Bob then puts the following content into his `stoic.toml`:

  ```toml
  [nvim]
  target = "~/.config/nvim"
  ```

  and after running `stoic` inside `my-configs/` the program creates the symlinks

  ```
  /home/bob/.config
  └── nvim
     └── init.lua -> /home/bob/my-configs/nvim/init.lua
  ```

- `recursive` (optional): whether or not the program should create symlinks for
  subdirectories present in `my-configs/program`. If the variable isn't set, the
  program will assume that `recursive = false`.

  Suppose that Bob has a more complex Neovim configuration layout:

  ```
  nvim
  ├── lua
  │  └── bob
  │     ├── plugins.lua
  │     └── settings.lua
  └── init.lua
  ```

  he can then enable the recursive option for the `nvim` node:

  ```toml
  [nvim]
  target = "~/.config/nvim"
  recursive = true
  ```

  and after running `stoic` inside `my-configs` the program should create the
  following symlinks:

  ```
  /home/bob/.config
  └── nvim
     ├── lua
     │  └── bob
     │     ├── plugins.lua -> /home/bob/my-configs/nvim/lua/bob/plugins.lua
     │     └── settings.lua -> /home/bob/my-configs/nvim/lua/bob/settings.lua
     └── init.lua -> /home/bob/my-configs/nvim/init.lua
  ```

- `src` (optional): string containing the path to the configuration
  directory to be the source of the symlinks, if the variable isn't set, the
  program will assume that the relative path to `dotfile.toml` is `"./key"` for
  the corresponding `[key]` in the config file.

  If Bob wants to store all his Tmux-related configurations in a single
  directory but does not want all files to go be symlinked to the same relative
  target directory:




  He can obtain this by adding the following to his config file:

  ```toml
  [tmux]
  target = "~/.config/tmux"

  [tmux_scripts]
  src = "tmux/scripts"
  target = "~/.local/bin"
  ```

  the resulting symlinks are:

  ```
  /home/bob/.config
  └── tmux
     ├── scripts
     │  └── sessions.sh -> /home/bob/my-configs/tmux/scripts/sessions.sh
     └── tmux.conf -> /home/bob/my-configs/tmux/tmux.conf
  /home/bob/.local
  └── bin
     └── sessions.sh -> /home/bob/my-configs/tmux/scripts/sessions.sh
  ```

# Alternatives

- [GNU stow](https://www.gnu.org/software/stow/).
