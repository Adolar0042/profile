use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};

use crate::rule::ProfileRef;
use crate::Action::{Apply, Completions, List, Show};

use self::profile::apply::apply;
use self::profile::list::list;
use self::profile::show::show;

mod config;
mod profile;
mod rule;
mod url;

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Lists all configured profiles.
    List {
        /// Shows only the name of the profiles
        #[clap(short, long)]
        short: bool,
    },
    /// Shows a profile in TOML format.
    Show { profile: String },
    /// Apply a profile.
    Apply { profile: Option<String> },
    /// Generate shell completions for the given shell.
    Completions {
        #[clap(value_enum)]
        shell: Shell,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cmd {
    #[clap(subcommand)]
    action: Action,
}

fn main() {
    let cmd = Cmd::parse();

    match cmd.action {
        List { short } => list(short).unwrap(),
        Show { profile: name } => show(ProfileRef { name }).unwrap(),
        Apply { profile: name } => apply(name).unwrap(),
        Completions { shell } => {
            let mut cmd = Cmd::command();
            generate(
                shell,
                &mut cmd,
                env!("CARGO_PKG_NAME"),
                &mut std::io::stdout(),
            );
        }
    }
}
