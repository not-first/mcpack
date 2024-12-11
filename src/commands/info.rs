use anyhow::{Context, Result};
use console::style;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::ZipArchive;

use crate::pack_formats;

#[derive(Debug)]
struct DatapackInfo {
    name: String,
    description: String,
    pack_format: u8,
    supported_formats: Vec<u8>,
    namespaces: HashMap<String, NamespaceInfo>,
}

#[derive(Debug, Default)]
struct NamespaceInfo {
    functions: usize,
    advancements: usize,
    recipes: usize,
    loot_tables: usize,
    predicates: usize,
    tags: usize,
    world_gen: bool,
}

pub fn run(command: &crate::cli::Commands) -> Result<()> {
    if let crate::cli::Commands::Info { path } = command {
        match path {
            Some(zip_path) => {
                // Construct the full zip path
                let mut full_path = String::from(zip_path);
                if !full_path.ends_with(".zip") {
                    full_path.push_str(".zip");
                }

                // Check if the file exists
                if !std::path::Path::new(&full_path).exists() {
                    anyhow::bail!("Zip file not found: {}", full_path);
                }

                let file = fs::File::open(&full_path)
                    .with_context(|| format!("Failed to open zip file: {}", full_path))?;
                let mut archive = ZipArchive::new(file)
                    .with_context(|| format!("Failed to read zip archive: {}", full_path))?;

                let pack_mcmeta_content = find_pack_mcmeta_in_zip(&mut archive)?;
                let info = collect_info_from_zip(&pack_mcmeta_content, &mut archive, &full_path)?;
                display_info(&info);
            }
            None => {
                let pack_mcmeta = PathBuf::from("pack.mcmeta");
                if !pack_mcmeta.exists() {
                    anyhow::bail!("Not in a datapack directory (pack.mcmeta not found)");
                }
                let info = collect_info(&pack_mcmeta)?;
                display_info(&info);
            }
        }
    }

    Ok(())
}

fn find_pack_mcmeta_in_zip(archive: &mut ZipArchive<fs::File>) -> Result<String> {
    let mut pack_mcmeta_content = None;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();

        if name == "pack.mcmeta" || name.ends_with("/pack.mcmeta") {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            pack_mcmeta_content = Some(content);
            break;
        }
    }

    pack_mcmeta_content.context("pack.mcmeta not found in zip archive")
}

fn collect_info_from_zip(
    pack_mcmeta_content: &str,
    archive: &mut ZipArchive<fs::File>,
    zip_path: &str,
) -> Result<DatapackInfo> {
    let mcmeta: Value =
        serde_json::from_str(pack_mcmeta_content).context("Failed to parse pack.mcmeta")?;

    let pack = mcmeta
        .get("pack")
        .context("Invalid pack.mcmeta: missing 'pack' object")?;

    let pack_format = pack
        .get("pack_format")
        .context("Missing pack_format")?
        .as_u64()
        .context("Invalid pack_format")? as u8;

    let mut supported_formats = vec![pack_format];

    if let Some(formats) = pack.get("supported_formats") {
        match formats {
            Value::Array(arr) => {
                supported_formats = arr.iter().map(|v| v.as_u64().unwrap_or(0) as u8).collect();
            }
            Value::Object(obj) => {
                if let (Some(min), Some(max)) = (
                    obj.get("min_inclusive").and_then(|v| v.as_u64()),
                    obj.get("max_inclusive").and_then(|v| v.as_u64()),
                ) {
                    supported_formats = (min as u8..=max as u8).collect();
                }
            }
            _ => {}
        }
    }

    let description = match pack.get("description") {
        Some(Value::String(s)) => s.to_string(),
        Some(Value::Array(arr)) => {
            arr.iter()
                .map(|component| {
                    match component {
                        Value::String(s) => s.to_string(),
                        Value::Object(obj) => {
                            if let Some(text) = obj.get("text").and_then(|t| t.as_str()) {
                                let color = obj.get("color").and_then(|c| c.as_str()).unwrap_or("");
                                match color {
                                    "" => text.to_string(),
                                    c if c.starts_with('#') => style(text).color256(24).to_string(),
                                    "gray" => style(text).dim().to_string(),
                                    _ => style(text).color256(24).to_string(), // Default to a nice color if we don't recognize it
                                }
                            } else {
                                String::new()
                            }
                        }
                        _ => String::new(),
                    }
                })
                .collect::<String>()
        }
        Some(Value::Object(obj)) => {
            if let Some(text) = obj.get("text").and_then(|t| t.as_str()) {
                let color = obj.get("color").and_then(|c| c.as_str()).unwrap_or("");
                match color {
                    "" => text.to_string(),
                    c if c.starts_with('#') => style(text).color256(24).to_string(),
                    "gray" => style(text).dim().to_string(),
                    _ => style(text).color256(24).to_string(),
                }
            } else {
                "Invalid description format".to_string()
            }
        }
        _ => "Invalid description".to_string(),
    };

    let name = Path::new(zip_path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let mut namespaces = HashMap::new();
    let mut current_namespace = String::new();
    let mut current_info = NamespaceInfo::default();

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let path = file.name();

        if let Some(data_path) = path.strip_prefix("data/") {
            let parts: Vec<&str> = data_path.split('/').collect();
            if parts.len() >= 2 {
                let namespace = parts[0].to_string();

                if !current_namespace.is_empty() && namespace != current_namespace {
                    if current_info.has_content() {
                        namespaces.insert(current_namespace.clone(), current_info);
                    }
                    current_info = NamespaceInfo::default();
                }

                current_namespace = namespace;

                if path.contains("/worldgen/") {
                    current_info.world_gen = true;
                }

                match parts.last() {
                    Some(filename) if filename.ends_with(".mcfunction") => {
                        current_info.functions += 1
                    }
                    Some(filename) if filename.ends_with(".json") => {
                        if path.contains("/advancement/") {
                            current_info.advancements += 1;
                        } else if path.contains("/recipe/") {
                            current_info.recipes += 1;
                        } else if path.contains("/loot_table/") {
                            current_info.loot_tables += 1;
                        } else if path.contains("/predicate/") {
                            current_info.predicates += 1;
                        } else if path.contains("/tags/") {
                            current_info.tags += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if !current_namespace.is_empty() && current_info.has_content() {
        namespaces.insert(current_namespace, current_info);
    }

    Ok(DatapackInfo {
        name,
        description,
        pack_format,
        supported_formats,
        namespaces,
    })
}

fn collect_info(pack_mcmeta_path: &Path) -> Result<DatapackInfo> {
    let content = fs::read_to_string(pack_mcmeta_path).context("Failed to read pack.mcmeta")?;
    let mcmeta: Value = serde_json::from_str(&content).context("Failed to parse pack.mcmeta")?;

    let pack = mcmeta
        .get("pack")
        .context("Invalid pack.mcmeta: missing 'pack' object")?;

    let pack_format = pack
        .get("pack_format")
        .context("Missing pack_format")?
        .as_u64()
        .context("Invalid pack_format")? as u8;

    let mut supported_formats = vec![pack_format];

    if let Some(formats) = pack.get("supported_formats") {
        match formats {
            Value::Array(arr) => {
                supported_formats = arr.iter().map(|v| v.as_u64().unwrap_or(0) as u8).collect();
            }
            Value::Object(obj) => {
                if let (Some(min), Some(max)) = (
                    obj.get("min_inclusive").and_then(|v| v.as_u64()),
                    obj.get("max_inclusive").and_then(|v| v.as_u64()),
                ) {
                    supported_formats = (min as u8..=max as u8).collect();
                }
            }
            _ => {}
        }
    }

    let description = match pack.get("description") {
        Some(Value::String(s)) => s.to_string(),
        Some(Value::Array(arr)) => {
            arr.iter()
                .map(|component| {
                    match component {
                        Value::String(s) => s.to_string(),
                        Value::Object(obj) => {
                            if let Some(text) = obj.get("text").and_then(|t| t.as_str()) {
                                let color = obj.get("color").and_then(|c| c.as_str()).unwrap_or("");
                                match color {
                                    "" => text.to_string(),
                                    c if c.starts_with('#') => style(text).color256(24).to_string(),
                                    "gray" => style(text).dim().to_string(),
                                    _ => style(text).color256(24).to_string(), // Default to a nice color if we don't recognize it
                                }
                            } else {
                                String::new()
                            }
                        }
                        _ => String::new(),
                    }
                })
                .collect::<String>()
        }
        Some(Value::Object(obj)) => {
            if let Some(text) = obj.get("text").and_then(|t| t.as_str()) {
                let color = obj.get("color").and_then(|c| c.as_str()).unwrap_or("");
                match color {
                    "" => text.to_string(),
                    c if c.starts_with('#') => style(text).color256(24).to_string(),
                    "gray" => style(text).dim().to_string(),
                    _ => style(text).color256(24).to_string(),
                }
            } else {
                "Invalid description format".to_string()
            }
        }
        _ => "Invalid description".to_string(),
    };

    let name = std::env::current_dir()?
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let data_dir = Path::new("data");
    let mut namespaces = HashMap::new();

    if data_dir.exists() {
        for entry in fs::read_dir(&data_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let namespace = entry.file_name().to_string_lossy().to_string();
                let namespace_info = collect_namespace_info(&entry.path())?;
                if namespace_info.has_content() {
                    namespaces.insert(namespace, namespace_info);
                }
            }
        }
    }

    Ok(DatapackInfo {
        name,
        description,
        pack_format,
        supported_formats,
        namespaces,
    })
}

impl NamespaceInfo {
    fn has_content(&self) -> bool {
        self.functions > 0
            || self.advancements > 0
            || self.recipes > 0
            || self.loot_tables > 0
            || self.predicates > 0
            || self.tags > 0
            || self.world_gen
    }
}

fn collect_namespace_info(namespace_path: &Path) -> Result<NamespaceInfo> {
    let mut info = NamespaceInfo::default();

    for entry in WalkDir::new(namespace_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let relative = path
            .strip_prefix(namespace_path)
            .unwrap()
            .to_string_lossy()
            .to_string();

        if relative.starts_with("worldgen/") {
            info.world_gen = true;
        }

        match path.extension().and_then(|s| s.to_str()) {
            Some("mcfunction") => info.functions += 1,
            Some("json") => {
                if relative.starts_with("advancement/") {
                    info.advancements += 1;
                } else if relative.starts_with("recipe/") {
                    info.recipes += 1;
                } else if relative.starts_with("loot_table/") {
                    info.loot_tables += 1;
                } else if relative.starts_with("predicate/") {
                    info.predicates += 1;
                } else if relative.starts_with("tags/") {
                    info.tags += 1;
                }
            }
            _ => {}
        }
    }

    Ok(info)
}

fn display_info(info: &DatapackInfo) {
    println!(
        "\n{} {}",
        style("📦").cyan(),
        style(&info.name).cyan().bold()
    );
    println!("{}", style(&info.description).italic());

    let valid_formats: Vec<u8> = info
        .supported_formats
        .iter()
        .filter(|&&f| pack_formats::is_valid_format(f))
        .copied()
        .collect();
    let versions = pack_formats::get_version_range(&valid_formats);
    let version_range = pack_formats::format_version_range(&versions);

    println!(
        "\n{} Pack Format{}: {} ({})",
        style("📝").green(),
        if valid_formats.len() > 1 { "s" } else { "" },
        valid_formats
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(", "),
        style(version_range).yellow()
    );

    for (namespace, info) in &info.namespaces {
        println!(
            "\n{} {} {}",
            style("📂").blue(),
            style("Namespace:").blue().bold(),
            style(namespace).white()
        );

        if info.functions > 0 {
            println!("  {} Functions: {}", style("↪").dim(), info.functions);
        }
        if info.advancements > 0 {
            println!("  {} Advancements: {}", style("↪").dim(), info.advancements);
        }
        if info.recipes > 0 {
            println!("  {} Recipes: {}", style("↪").dim(), info.recipes);
        }
        if info.loot_tables > 0 {
            println!("  {} Loot Tables: {}", style("↪").dim(), info.loot_tables);
        }
        if info.predicates > 0 {
            println!("  {} Predicates: {}", style("↪").dim(), info.predicates);
        }
        if info.tags > 0 {
            println!("  {} Tags: {}", style("↪").dim(), info.tags);
        }

        if info.world_gen {
            println!(
                "  {} {}",
                style("↪").dim(),
                style("This namespace alters world generation")
                    .green()
                    .italic()
            );
        }
    }
    println!();
}

fn color_to_code(color: &str) -> &str {
    match color {
        "black" => "0",
        "dark_blue" => "1",
        "dark_green" => "2",
        "dark_aqua" => "3",
        "dark_red" => "4",
        "dark_purple" => "5",
        "gold" => "6",
        "gray" => "7",
        "dark_gray" => "8",
        "blue" => "9",
        "green" => "a",
        "aqua" => "b",
        "red" => "c",
        "light_purple" => "d",
        "yellow" => "e",
        "white" => "f",
        _ => "f",
    }
}
