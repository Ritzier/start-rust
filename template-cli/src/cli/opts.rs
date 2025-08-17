use clap_derive::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Opts {
    /// Verbosity (none: info, errors & warnings, -v: verbose, -vv: very verbose)
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
