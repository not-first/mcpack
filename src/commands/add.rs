use crate::elements::{get_sample_content, is_valid_element_type, ELEMENT_TYPES};
use anyhow::{Context, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::fs;
use std::path::PathBuf;

pub fn run(command: &crate::cli::Commands) -> Result<()> {
    if let crate::cli::Commands::Add {
        element,
        path,
        namespace,
        name,
        force,
    } = command
    {
        let theme = &ColorfulTheme::default();

        // prompt for element_type if not provided
        let element_type = if let Some(et) = element {
            et.clone()
        } else {
            // list of available element types
            let element_names: Vec<&str> = ELEMENT_TYPES.iter().map(|(name, _)| *name).collect();
            let selection = Select::with_theme(theme)
                .with_prompt("Select element type to add")
                .items(&element_names)
                .interact()?;
            element_names[selection].to_string()
        };

        // validate element type
        if !is_valid_element_type(&element_type) {
            let valid_types = ELEMENT_TYPES
                .iter()
                .map(|(name, _)| *name)
                .collect::<Vec<_>>()
                .join(", ");
            anyhow::bail!("Invalid element type. Supported types are: {}", valid_types);
        }

        // **Move flags_used computation before unwrapping `name`**
        let flags_used = element.is_some() || name.is_some();

        // prompt for name if not provided
        let name = if let Some(n) = name {
            n.clone()
        } else {
            Input::with_theme(theme)
                .with_prompt("Enter name for the new file")
                .interact_text()
                .context("Failed to get file name")?
        };

        let root_dir = if let Some(p) = path {
            PathBuf::from(p)
        } else {
            std::env::current_dir()?
        };

        // verify it's a datapack directory
        if !root_dir.join("pack.mcmeta").exists() {
            anyhow::bail!("Not a datapack directory (pack.mcmeta not found)");
        }

        // get or select namespace
        let namespace = if let Some(ns) = namespace {
            ns.clone()
        } else {
            // look for existing namespaces
            let data_dir = root_dir.join("data");
            let mut namespaces: Vec<String> = if data_dir.exists() {
                fs::read_dir(&data_dir)?
                    .filter_map(|entry| {
                        entry.ok().and_then(|e| {
                            if e.file_type().ok()?.is_dir() {
                                Some(e.file_name().to_string_lossy().to_string())
                            } else {
                                None
                            }
                        })
                    })
                    .collect()
            } else {
                Vec::new()
            };

            // if flags are used without specifying a namespace, exclude 'minecraft' from consideration
            if flags_used {
                namespaces.retain(|ns| ns != "minecraft");
            }

            if namespaces.is_empty() {
                // if no namespaces are present, prompt for namespace (including 'minecraft' if flags are used)
                if flags_used {
                    let mut all_namespaces = Vec::new();
                    if data_dir.exists() {
                        all_namespaces = fs::read_dir(&data_dir)?
                            .filter_map(|entry| {
                                entry.ok().and_then(|e| {
                                    if e.file_type().ok()?.is_dir() {
                                        Some(e.file_name().to_string_lossy().to_string())
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect();
                    }
                    // include 'minecraft' in the prompt options only if flags are used
                    let selection = Select::with_theme(theme)
                        .with_prompt("Select namespace to add the element to")
                        .items(&all_namespaces)
                        .interact()?;
                    all_namespaces[selection].clone()
                } else {
                    // non-flagged command behavior
                    Input::with_theme(theme)
                        .with_prompt("Enter namespace name")
                        .interact_text()?
                }
            } else if namespaces.len() == 1 {
                // only one non-minecraft namespace exists, use it
                namespaces[0].clone()
            } else {
                if flags_used {
                    // multiple namespaces exist, prompt user excluding 'minecraft'
                    let selection = Select::with_theme(theme)
                        .with_prompt("Select namespace to add the element to")
                        .items(&namespaces)
                        .interact()?;
                    namespaces[selection].clone()
                } else {
                    // multiple namespaces exist, include 'minecraft' in the prompt options
                    let mut prompt_namespaces = namespaces.clone();
                    prompt_namespaces.push("minecraft".to_string());
                    let selection = Select::with_theme(theme)
                        .with_prompt("Select namespace to add the element to")
                        .items(&prompt_namespaces)
                        .interact()?;
                    prompt_namespaces[selection].clone()
                }
            }
        };

        let (_, extension) = ELEMENT_TYPES
            .iter()
            .find(|(name, _)| *name == element_type)
            .unwrap();

        let data_type_dir = root_dir.join("data").join(&namespace).join(&element_type);

        let file_path = data_type_dir.join(format!("{}{}", name, extension));

        // create the parent directories if they don't exist
        if let Some(parent_dir) = file_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        // check if the file already exists
        if file_path.exists() && !*force {
            let confirm = Confirm::with_theme(theme)
                .with_prompt(format!(
                    "File '{}' already exists. Overwrite?",
                    file_path.display()
                ))
                .default(false)
                .interact()?;

            if !confirm {
                println!(
                    "{} Skipped creating '{}'",
                    style("⚠️").yellow(),
                    file_path.display()
                );
                return Ok(());
            }
        }

        fs::write(&file_path, get_sample_content(&element_type))?;

        println!(
            "\n{} Created {} '{}'",
            style("✓").green(),
            style(&element_type).cyan(),
            style(
                file_path
                    .strip_prefix(&root_dir.join("data").join(&namespace).join(&element_type))?
                    .display()
            )
            .white()
        );
    }

    Ok(())
}
