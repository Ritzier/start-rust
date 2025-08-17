use std::env;

use clap::Parser;
use rust_cli_template::{Cli, Result};

{% if use_tokio -%}#[tokio::main]
async {% endif -%}fn main() -> Result<()> {
    color_eyre::install()?;

    let mut args: Vec<String> = env::args().collect();

    // Remove `CARGO_PKG_NAME` from first argument
    if args
        .get(1)
        .map(|arg| arg == env!("CARGO_PKG_NAME"))
        .unwrap_or(false)
    {
        args.remove(1);
    }

    // Parse args
    let args = Cli::parse_from(&args);

    // Setup logger
    let _verbose = args.verbose();

    args.run(){% if use_tokio -%}.await{% endif -%}
}
