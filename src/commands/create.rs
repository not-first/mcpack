use crate::cli::Commands;
use crate::pack_formats::{self, PACK_FORMATS};
use anyhow::{Context, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect};
use rfd::FileDialog;
use serde::Serialize;
use serde_json;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct PackSettings {
    directory: PathBuf,
    name: String,
    description: String,
    icon_path: Option<String>,
    pack_formats: Vec<u8>,
    include_minecraft_namespace: bool,
    minecraft_tags: Vec<String>,
    custom_namespace: Option<String>,
    custom_namespace_folders: Vec<String>,
}

#[derive(Serialize)]
struct PackMcmeta {
    pack: Pack,
}

#[derive(Serialize)]
struct Pack {
    description: String,
    pack_format: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_formats: Option<SupportedFormatsType>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum SupportedFormatsType {
    Array(Vec<u8>),
    Object {
        min_inclusive: u8,
        max_inclusive: u8,
    },
}

pub fn run(args: &Commands) -> Result<()> {
    let Commands::Create {
        name,
        description,
        icon,
        format,
        minecraft,
        minecraft_load,
        minecraft_tick,
        namespace,
        folders,
    } = args
    else {
        unreachable!("create::run should only be called with Create command");
    };

    // Only need to validate format arguments
    if let Some(formats) = format {
        for &f in formats {
            if !pack_formats::is_valid_format(f) {
                anyhow::bail!(
                    "Invalid pack format: {}. Valid formats are: {}",
                    f,
                    pack_formats::get_formats_string()
                );
            }
        }
    }

    let theme = ColorfulTheme::default();
    let settings = collect_settings(
        &theme,
        CreateArgs {
            name: name.clone(),
            description: description.clone(),
            icon_path: icon.clone(),
            pack_formats: format.clone(), // Now we can use it directly
            include_minecraft: *minecraft,
            minecraft_tags: if *minecraft {
                let mut tags = Vec::new();
                if *minecraft_load {
                    tags.push("load.mcfunction".to_string());
                }
                if *minecraft_tick {
                    tags.push("tick.mcfunction".to_string());
                }
                Some(tags)
            } else {
                None
            },
            custom_namespace: namespace.clone(),
            namespace_folders: folders.clone(), // Now we can use it directly
        },
    )?;

    create_pack(settings)?;
    Ok(())
}

struct CreateArgs {
    name: Option<String>,
    description: Option<String>,
    icon_path: Option<String>,
    pack_formats: Option<Vec<u8>>, // Changed from Option<String>
    include_minecraft: bool,
    minecraft_tags: Option<Vec<String>>,
    custom_namespace: Option<String>,
    namespace_folders: Option<Vec<String>>, // Changed from Option<String>
}

fn collect_settings(theme: &ColorfulTheme, args: CreateArgs) -> Result<PackSettings> {
    let name = match args.name {
        Some(name) => name,
        None => Input::with_theme(theme)
            .with_prompt("Enter Datapack name")
            .default("my-datapack".to_string())
            .interact_text()
            .context("Failed to get datapack name")?,
    };

    let directory = std::env::current_dir()
        .context("Failed to get current directory")?
        .join(name.clone());

    let description = match args.description {
        Some(description) => description,
        None => Input::with_theme(theme)
            .with_prompt("Datapack description")
            .default("A newly created datapack".to_string())
            .interact_text()
            .context("Failed to get datapack description")?,
    };

    let icon_path = match args.icon_path {
        Some(icon_path) => Some(icon_path),
        None => {
            let pick_icon = Confirm::with_theme(theme)
                .with_prompt("Do you want to add a pack icon?")
                .default(false)
                .interact()
                .context("Failed to get icon confirmation")?;

            if pick_icon {
                let file = FileDialog::new()
                    .add_filter("PNG Image", &["png"])
                    .set_title("Select pack icon")
                    .pick_file();

                file.map(|path| path.to_string_lossy().to_string())
            } else {
                None
            }
        }
    };

    // Pack format selection
    let pack_formats = match args.pack_formats {
        Some(pack_formats) => pack_formats, // Validation already done
        None => {
            let format_strings: Vec<String> = PACK_FORMATS
                .iter()
                .map(|&f| {
                    let info = pack_formats::get_version_info(f).unwrap();
                    format!("Format {} ({})", f, info.versions.join(", "))
                })
                .collect();

            let selected_formats = MultiSelect::with_theme(theme)
                .with_prompt("Select pack format(s)")
                .items(&format_strings)
                .defaults(&[true])
                .interact()
                .context("Failed to select pack formats")?;

            let pack_formats: Vec<u8> = selected_formats.iter().map(|&i| PACK_FORMATS[i]).collect();

            if pack_formats.is_empty() {
                anyhow::bail!("No pack formats selected");
            }

            pack_formats
        }
    };

    // Datapack settings
    let include_minecraft_namespace = args.include_minecraft
        || Confirm::with_theme(theme)
            .with_prompt("Include minecraft namespace files?")
            .default(false)
            .interact()
            .context("Failed to get minecraft namespace confirmation")?;

    let minecraft_tags = match args.minecraft_tags {
        Some(tags) => tags,
        None => {
            if include_minecraft_namespace {
                let tag_options = vec!["load.mcfunction", "tick.mcfunction"];
                let selected_tags = MultiSelect::with_theme(theme)
                    .with_prompt("Select minecraft tags to include")
                    .items(&tag_options)
                    .interact()
                    .context("Failed to select minecraft tags")?;

                selected_tags
                    .iter()
                    .map(|&i| tag_options[i].to_string())
                    .collect()
            } else {
                Vec::new()
            }
        }
    };

    // Custom namespace prompt
    let custom_namespace = match args.custom_namespace {
        Some(custom_namespace) => Some(custom_namespace),
        None => {
            let input: String = Input::with_theme(theme)
                .with_prompt("Enter custom namespace")
                .allow_empty(true)
                .interact_text()
                .context("Failed to get custom namespace")?;

            if input.trim().is_empty() {
                None
            } else {
                Some(input)
            }
        }
    };

    let custom_namespace_folders = match args.namespace_folders {
        Some(namespace_folders) => namespace_folders,
        None => {
            if custom_namespace.is_some() {
                let folder_options = vec![
                    "function",
                    "advancement",
                    "tags",
                    "recipe",
                    "loot_table",
                    "predicate",
                ];

                let selected_folders = MultiSelect::with_theme(theme)
                    .with_prompt("Select starter folders for custom namespace")
                    .items(&folder_options)
                    .interact()
                    .context("Failed to select starter folders")?;

                selected_folders
                    .iter()
                    .map(|&i| folder_options[i].to_string())
                    .collect()
            } else {
                Vec::new()
            }
        }
    };

    Ok(PackSettings {
        directory,
        name,
        description,
        icon_path,
        pack_formats,
        include_minecraft_namespace,
        minecraft_tags,
        custom_namespace,
        custom_namespace_folders,
    })
}

fn create_pack(pack_settings: PackSettings) -> Result<()> {
    if pack_settings.directory.exists() {
        anyhow::bail!(
            "A datapack named '{}' already exists at {}",
            pack_settings.name,
            pack_settings.directory.display()
        );
    }

    std::fs::create_dir_all(&pack_settings.directory)
        .context("Failed to create datapack directory")?;

    // Handle icon if provided
    if let Some(icon_path) = pack_settings.icon_path {
        let icon_source = PathBuf::from(&icon_path);
        if (!icon_source.exists()) {
            anyhow::bail!("Selected icon file does not exist: {}", icon_path);
        }

        // Verify it's a PNG file
        let extension = icon_source
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        if extension.to_lowercase() != "png" {
            anyhow::bail!("Icon must be a PNG file");
        }

        // Copy the icon to pack.png in the datapack directory
        fs::copy(&icon_source, pack_settings.directory.join("pack.png"))
            .context("Failed to copy icon file")?;
    }

    let latest_format = *pack_settings.pack_formats.iter().max().unwrap();

    let supported_formats = if pack_settings.pack_formats.len() > 1 {
        let min = *pack_settings.pack_formats.iter().min().unwrap();
        let max = *pack_settings.pack_formats.iter().max().unwrap();

        // Only get valid formats in the range
        let formats_in_range = pack_formats::get_formats_in_range(min, max);

        // Check if selected formats exactly match the valid formats in range
        let selected_set: std::collections::HashSet<_> =
            pack_settings.pack_formats.iter().collect();
        let range_set: std::collections::HashSet<_> = formats_in_range.iter().collect();

        if selected_set == range_set && formats_in_range.len() >= 3 {
            Some(SupportedFormatsType::Object {
                min_inclusive: min,
                max_inclusive: max,
            })
        } else {
            Some(SupportedFormatsType::Array(
                pack_settings.pack_formats.clone(),
            ))
        }
    } else {
        None
    };

    let pack_mcmeta = PackMcmeta {
        pack: Pack {
            pack_format: latest_format,
            description: pack_settings.description,
            supported_formats,
        },
    };

    let pack_mcmeta_json =
        serde_json::to_string_pretty(&pack_mcmeta).context("Failed to serialize pack.mcmeta")?;

    std::fs::write(
        pack_settings.directory.join("pack.mcmeta"),
        pack_mcmeta_json,
    )
    .context("Failed to write pack.mcmeta")?;

    // Create data folder structure
    let data_dir = pack_settings.directory.join("data");
    fs::create_dir_all(&data_dir).context("Failed to create data directory")?;

    if pack_settings.include_minecraft_namespace {
        let minecraft_tags_dir = data_dir.join("minecraft/tags/function");
        fs::create_dir_all(&minecraft_tags_dir)
            .context("Failed to create minecraft tags directory")?;

        // Create selected tag files
        for tag in &pack_settings.minecraft_tags {
            let tag_name = tag.strip_suffix(".mcfunction").unwrap_or(tag);
            let tag_content = serde_json::json!({
                "values": []
            });
            fs::write(
                minecraft_tags_dir.join(format!("{}.json", tag_name)),
                serde_json::to_string_pretty(&tag_content)?,
            )
            .with_context(|| format!("Failed to create {} tag file", tag_name))?;
        }
    }

    // Handle custom namespace if provided
    if let Some(namespace) = &pack_settings.custom_namespace {
        let namespace_dir = data_dir.join(namespace);
        fs::create_dir_all(&namespace_dir)
            .with_context(|| format!("Failed to create namespace directory for {}", namespace))?;

        // Create selected folders and their starter files
        for folder in &pack_settings.custom_namespace_folders {
            let folder_path = namespace_dir.join(folder);
            fs::create_dir_all(&folder_path)
                .with_context(|| format!("Failed to create {} folder", folder))?;

            // Create starter files based on folder type
            match folder.as_str() {
                "function" => {
                    let main_mcfunction = folder_path.join("main.mcfunction");
                    fs::write(main_mcfunction, "").context("Failed to create main.mcfunction")?;
                }
                "advancement" => {
                    let example_advancement = folder_path.join("advancement.json");
                    let content = serde_json::json!({
                      "criteria": {}
                    });
                    fs::write(example_advancement, serde_json::to_string_pretty(&content)?)
                        .context("Failed to create example advancement")?;
                }
                "tags" => {
                    fs::create_dir_all(&folder_path).context("Failed to create tags folder")?;
                }
                "recipe" => {
                    let example_recipe = folder_path.join("recipe.json");
                    let content = serde_json::json!({
                      "type": ""
                    });
                    fs::write(example_recipe, serde_json::to_string_pretty(&content)?)
                        .context("Failed to create example recipe")?;
                }
                "loot_table" => {
                    let example_loot = folder_path.join("loot_table.json");
                    let content = serde_json::json!({});
                    fs::write(example_loot, serde_json::to_string_pretty(&content)?)
                        .context("Failed to create example loot table")?;
                }
                "predicate" => {
                    let example_predicate = folder_path.join("predicate.json");
                    let content = serde_json::json!({
                      "condition": ""
                    });
                    fs::write(example_predicate, serde_json::to_string_pretty(&content)?)
                        .context("Failed to create example predicate")?;
                }
                _ => {}
            }
        }
    }

    println!(
        "\n{} {} '{}'",
        style("✨").cyan(),
        style("Successfully created datapack").green().bold(),
        style(&pack_settings.name).cyan().bold()
    );
    println!(
        "{} {}",
        "📂",
        style(pack_settings.directory.display().to_string())
            .blue()
            .underlined()
    );

    Ok(())
}
