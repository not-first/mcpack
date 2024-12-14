use anyhow::Result;
use clap::Parser;
use console::style;
use mcpack::cli::{Cli, Commands};
use mcpack::commands;

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
