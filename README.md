# Musht

## About

Musht is a simple wrapper for the mosh client written in rust.
Its main goal is to provide support for loading port information from dns records, but also features a few other tweaks. Musht does not need to be installed on the server, and will work just fine with a standard mosh install, but to take full advantage of it's features you will need a domain and the ability to configure dns records.

## Features

### Current

- Resolve ports from txt records, allowing both ssh ports and mosh ports to be stored in the domain

- allow ssh ports to be passed with [user]@host:[port] syntax instead of requiring ssh ports to be passed as an ssh argument

### Planned

- fallback to ssh if server doesn't have/can't start mosh

- if tmux is installed and configured, automatically enable support for scrolling / history

## Installation

At this time, no prebuilt binaries are published, so you'll need to compile them yourself.

First, make sure you have rust installed. The recommended way to this is through [rustup](https://rustup.rs/), but there are alternate ways if you would like better integration with your package manager.

You will also need to make sure you've added $HOME/.cargo/bin to your path.

Musht also requires [mosh](https://mosh.org/) to run, so make sure you have it installed.

From here there are two installation methods. If you're not sure which one to pick, follow the install script method.

### Install Script

1. clone the repository `git clone https://github.com/elliotnash/musht.git`

2. enter musht directory `cd musht`

3. make install script executable `chmod 777 *install.sh`

4. run install `./install.sh`

### Manual Install

the install script edits your .bashrc or .zshrc, so if you would like complete control over the installation, follow this

1. clone the repository `git clone https://github.com/elliotnash/musht.git`

2. enter musht directory `cd musht`

3. build and install binary to ~/.cargo/bin `cargo install --force --path .`

4. to enable completions, setup `complete -C __musht_completions musht` to be run each time your shell starts (etc in your .bashrc or .zshrc)

## Dns Setup

To take adventage of musht's dns resolving, you'll need to have a record like an a, aaaa, or cname pointing to your servers ip, and you'll also need a txt record with the same name with a value like this, replacing the ports with the ports you want mosh to use.
```json
{"ssh_port": "22", "mosh_ports": "60000:61000"}
```
ssh_port must be a single port, wheras mosh_ports should be a port range. mosh_ports *can* be set to a single port, but it is highly discouraged as each mosh connection runs on a seperate port, and a single port would only allow one simultaneous connection

## Uninstallation

run `./uninstall.sh` from the git directory

## License

Musht is licensed under GPL-3.0, see [LICENSE](LICENSE)
