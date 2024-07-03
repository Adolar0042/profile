# 🕵️‍♂️ Profile

A simple profile switcher for your terminal.

Hugely inspired by [ghr](https://github.com/siketyan/ghr/).

_Like totally didn't borrow like half their code, trust me bro._

## Installation

```sh
git clone https://github.com/Adolar0042/profile
cd profile
cargo install --path .
```
### Adding shell completions
Add the following to your shell's configuration file:
```sh
eval "$(profile completions)"
```

## Usage

Usage: profile <COMMAND>

Commands:
list         Lists all configured profiles
show         Shows a profile in TOML format
apply        Apply a profile
completions  Generate shell completions for the given shell
help         Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version

## Applying a profile

First, create a config file in your home directory: `~/.config/profile/profile.toml`

```toml
[profiles.default]
user.name = "Your Name"
user.email = "your_name@personal.example.com"

[profiles.company]
user.name = "Your Name (ACME Inc.)"
user.email = "your_name@company.example.com"

[[rules]]
profile.name = "company"
owner = "acme" # Applies company profiles to all repositories in `acme` org

[[rules]]
profile.name = "default"
```

Then, apply a profile:

```sh
profile apply default
# or
profile apply company
# or this to apply the profile based on rules
profile apply
```
