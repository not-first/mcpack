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
        #[arg(short, long, group = "icon_handling")]
        icon: Option<String>,

        /// Pack format(s) to support (space-separated list)
        #[arg(short = 'f', long = "format", num_args = 1.., value_delimiter = ' ')]
        format: Option<Vec<String>>,

        /// Include minecraft namespace folder
        #[arg(
            short = 'm',
            long,
            group = "mc_namespace",
            group = "minecraft_handling"
        )]
        minecraft: bool,

        /// Include template minecraft load.mcfunction tag
        #[arg(long = "load", requires = "minecraft")]
        minecraft_load: bool,

        /// Include template minecraft tick.mcfunction tag
        #[arg(long = "tick", requires = "minecraft")]
        minecraft_tick: bool,

        /// Custom namespace name
        #[arg(short, long, group = "namespace_handling")]
        namespace: Option<String>,

        /// Starter folder files to create in custom namespace (space-separated list)
        #[arg(short = 's', long = "starters", num_args = 1.., value_delimiter = ' ', requires = "namespace")]
        folders: Option<Vec<String>>,

        /// Output directory for the datapack
        #[arg(short = 'o', long)]
        output_dir: Option<String>,

        /// Force overwrite existing directory without prompting
        #[arg(short = 'F', long)]
        force: bool,

        /// Skip pack icon prompt
        #[arg(long = "no-icon", group = "icon_handling")]
        skip_icon: bool,

        /// Skip starter files creation for custom namespace
        #[arg(
            long = "no-starters",
            conflicts_with = "folders" // Removed from "namespace_handling" group
        )]
        skip_starters: bool,

        /// Skip minecraft tags selection
        #[arg(long = "no-minecraft-tags", group = "minecraft_handling")]
        skip_minecraft_tags: bool,
    },
    Info {
        /// Path to datapack folder or zip file
        path: Option<String>,

        /// Show only basic information (name, description, pack format)
        #[arg(long, group = "display_mode")]
        compact: bool,

        /// Show only pack.mcmeta information
        #[arg(long, group = "display_mode")]
        pack_info: bool,

        /// Show only namespace information
        #[arg(long, group = "display_mode")]
        namespaces: bool,
    },
    Zip {
        /// Path to datapack folder
        path: Option<String>,

        /// Input directory containing the unzipped datapack
        #[arg(short = 'i', long)]
        input_dir: Option<String>,

        /// Custom name for the output zip file
        #[arg(short = 'n', long)]
        name: Option<String>,

        /// Output directory for the zip file
        #[arg(short = 'o', long)]
        output_dir: Option<String>,

        /// Force overwrite existing zip file without prompting
        #[arg(short = 'F', long)]
        force: bool,
    },
    Add {
        /// Type of element to add (e.g., function, advancement, loot_table)
        #[arg(short, long)]
        element: Option<String>,

        /// Path to datapack directory
        #[arg(short, long)]
        path: Option<String>,

        /// Name of the namespace to add element to
        #[arg(short = 'x', long)]
        namespace: Option<String>,

        /// Path and name for the new file (supports subdirectories)
        #[arg(short, long)]
        name: Option<String>,

        /// Force overwrite existing files without prompting
        #[arg(short = 'F', long)]
        force: bool,
    },
}
