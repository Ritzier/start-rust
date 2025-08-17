use clap::Subcommand;
use clap_complete::Shell;

use super::Opts;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run
    Run(Opts),

    /// Shell completion
    Completions { shell: Shell },
}
