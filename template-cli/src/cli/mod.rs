use super::Result;

mod command;
use command::Command;

mod opts;
use opts::Opts;

use clap::{CommandFactory, Parser};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub{% if use_tokio -%} async{% endif -%} fn run(self) -> Result<()> {
        match self.command {
            Command::Run(_opts) => {}

            Command::Completions { shell } => {
                clap_complete::generate(
                    shell,
                    &mut Cli::command(),
                    env!("CARGO_PKG_NAME"),
                    &mut std::io::stdout(),
                );
            }
        }

        Ok(())
    }

    pub fn verbose(&self) -> u8 {
        match &self.command {
            Command::Run(opts) => opts.verbose,
            Command::Completions { .. } => 0,
        }
    }
}
