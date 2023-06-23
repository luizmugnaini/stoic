# stoic-dotfiles 🖇

<p align="center">
  <img src="stoic.png" alt="Random stoic bust" width="150"/>
</p>

Stoic is a CLI tool built using the Rust programming language. It aims to simplify the management of configuration files, also known as dotfiles, in a centralized manner. Drawing inspiration from the approach of the [GNU stow](https://www.gnu.org/software/stow/) utility, Stoic offers a flexible and efficient solution for organizing and deploying dotfiles across multiple systems.

With Stoic, users can effortlessly manage their dotfiles and maintain consistency across different environments. Whether you're a developer, sysadmin, or simply a power user, Stoic empowers you to streamline your configuration management workflow.

Key Features:

- Centralized Dotfile Management: Stoic allows you to keep all your dotfiles organized in a central directory, making it easy to version control and synchronize across multiple machines. No more scattered dotfiles across different locations!

- Simple and Intuitive CLI: Stoic provides a user-friendly command-line interface that makes dotfile management a breeze. It offers intuitive commands for adding, removing, and updating dotfiles, ensuring a seamless experience for both beginners and advanced users.

- Intelligent Symbolic Link Deployment: Stoic leverages symbolic links to deploy dotfiles to their respective locations. By creating symbolic links, the original dotfiles remain in the central directory, ensuring easy updates and preventing accidental file duplication.

- Customizable Configurations: Stoic understands that each user has unique requirements and preferences. It provides a configuration file where you can specify custom settings and behaviors to tailor Stoic to your specific needs.

Stoic aims to be a reliable and efficient tool for managing dotfiles, providing a robust foundation for maintaining a consistent and portable environment across different systems. It empowers users to focus on their work without worrying about the complexities of dotfile management, ultimately enabling them to be more productive and efficient.

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

- `target_path`: a string containing the path where the symlinks should be
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

- `is_recursive` (optional): whether or not the program should create symlinks for
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

- `config_path` (optional): string containing the path to the configuration
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
