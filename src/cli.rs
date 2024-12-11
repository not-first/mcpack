use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        /// Name of the datapack (creates folder with this name)
        name: Option<String>,

        /// Description of the datapack
        #[arg(short, long)]
        description: Option<String>,

        /// Path to pack icon (must be PNG)
        #[arg(short, long)]
        icon: Option<String>,

        /// Pack format(s) to support (space-separated list)
        #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
        format: Option<Vec<u8>>,

        /// Include minecraft namespace
        #[arg(short = 'm', long, group = "mc_namespace")]
        minecraft: bool,

        /// Include minecraft load.mcfunction tag
        #[arg(long = "load", requires = "minecraft")]
        minecraft_load: bool,

        /// Include minecraft tick.mcfunction tag
        #[arg(long = "tick", requires = "minecraft")]
        minecraft_tick: bool,

        /// Custom namespace
        #[arg(short, long)]
        namespace: Option<String>,

        /// Folders to create in custom namespace (space-separated list)
        #[arg(short = 'F', long, num_args = 1.., value_delimiter = ' ', requires = "namespace")]
        folders: Option<Vec<String>>,
    },
    Info,
}
