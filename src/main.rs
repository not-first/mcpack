mod cli;
mod commands;
mod pack_formats;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use console::style;

fn main() {
    if let Err(err) = run() {
        eprintln!("{} {}", style("error:").red().bold(), err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { .. } => commands::create::run(&cli.command)?,
        Commands::Info { .. } => commands::info::run(&cli.command)?,
    }

    Ok(())
}
