mod cli;
mod commands;
mod elements;
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

    // run the appropriate command
    match &cli.command {
        Commands::Create { .. } => commands::create::run(&cli.command)?,
        Commands::Info { .. } => commands::info::run(&cli.command)?,
        Commands::Zip { .. } => commands::zip::run(&cli.command)?,
        Commands::Add { .. } => commands::add::run(&cli.command)?,
    }

    Ok(())
}
