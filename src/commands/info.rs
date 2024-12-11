use anyhow::{Context, Result};
use console::style;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
    structures: usize,
    banner_patterns: usize,
    chat_types: usize,
    damage_types: usize,
    dimensions: usize,
    dimension_types: usize,
    enchantments: usize,
    enchantment_providers: usize,
    instruments: usize,
    item_modifiers: usize,
    jukebox_songs: usize,
    painting_variants: usize,
    trim_materials: usize,
    trim_patterns: usize,
    wolf_variants: usize,
    has_worldgen: bool,
}

impl NamespaceInfo {
    fn has_content(&self) -> bool {
        self.functions > 0
            || self.advancements > 0
            || self.recipes > 0
            || self.loot_tables > 0
            || self.predicates > 0
            || self.tags > 0
            || self.structures > 0
            || self.banner_patterns > 0
            || self.chat_types > 0
            || self.damage_types > 0
            || self.dimensions > 0
            || self.dimension_types > 0
            || self.enchantments > 0
            || self.enchantment_providers > 0
            || self.instruments > 0
            || self.item_modifiers > 0
            || self.jukebox_songs > 0
            || self.painting_variants > 0
            || self.trim_materials > 0
            || self.trim_patterns > 0
            || self.wolf_variants > 0
            || self.has_worldgen
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

        match path.extension().and_then(|s| s.to_str()) {
            Some("mcfunction") => info.functions += 1,
            Some("nbt") => info.structures += 1,
            Some("json") => match relative.split('/').next() {
                Some("advancements") => info.advancements += 1,
                Some("recipes") => info.recipes += 1,
                Some("loot_tables") => info.loot_tables += 1,
                Some("predicates") => info.predicates += 1,
                Some("tags") => info.tags += 1,
                Some("banner_pattern") => info.banner_patterns += 1,
                Some("chat_type") => info.chat_types += 1,
                Some("damage_type") => info.damage_types += 1,
                Some("dimension") => info.dimensions += 1,
                Some("dimension_type") => info.dimension_types += 1,
                Some("enchantment") => info.enchantments += 1,
                Some("enchantment_provider") => info.enchantment_providers += 1,
                Some("instrument") => info.instruments += 1,
                Some("item_modifier") => info.item_modifiers += 1,
                Some("jukebox_song") => info.jukebox_songs += 1,
                Some("painting_variant") => info.painting_variants += 1,
                Some("trim_material") => info.trim_materials += 1,
                Some("trim_pattern") => info.trim_patterns += 1,
                Some("wolf_variant") => info.wolf_variants += 1,
                Some("worldgen") => info.has_worldgen = true,
                _ => {}
            },
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
        if info.structures > 0 {
            println!("  {} Structures: {}", style("↪").dim(), info.structures);
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
        if info.banner_patterns > 0 {
            println!(
                "  {} Banner Patterns: {}",
                style("↪").dim(),
                info.banner_patterns
            );
        }
        if info.chat_types > 0 {
            println!("  {} Chat Types: {}", style("↪").dim(), info.chat_types);
        }
        if info.damage_types > 0 {
            println!("  {} Damage Types: {}", style("↪").dim(), info.damage_types);
        }
        if info.dimensions > 0 {
            println!("  {} Dimensions: {}", style("↪").dim(), info.dimensions);
        }
        if info.dimension_types > 0 {
            println!(
                "  {} Dimension Types: {}",
                style("↪").dim(),
                info.dimension_types
            );
        }
        if info.enchantments > 0 {
            println!("  {} Enchantments: {}", style("↪").dim(), info.enchantments);
        }
        if info.enchantment_providers > 0 {
            println!(
                "  {} Enchantment Providers: {}",
                style("↪").dim(),
                info.enchantment_providers
            );
        }
        if info.instruments > 0 {
            println!("  {} Instruments: {}", style("↪").dim(), info.instruments);
        }
        if info.item_modifiers > 0 {
            println!(
                "  {} Item Modifiers: {}",
                style("↪").dim(),
                info.item_modifiers
            );
        }
        if info.jukebox_songs > 0 {
            println!(
                "  {} Jukebox Songs: {}",
                style("↪").dim(),
                info.jukebox_songs
            );
        }
        if info.painting_variants > 0 {
            println!(
                "  {} Painting Variants: {}",
                style("↪").dim(),
                info.painting_variants
            );
        }
        if info.trim_materials > 0 {
            println!(
                "  {} Trim Materials: {}",
                style("↪").dim(),
                info.trim_materials
            );
        }
        if info.trim_patterns > 0 {
            println!(
                "  {} Trim Patterns: {}",
                style("↪").dim(),
                info.trim_patterns
            );
        }
        if info.wolf_variants > 0 {
            println!(
                "  {} Wolf Variants: {}",
                style("↪").dim(),
                info.wolf_variants
            );
        }
        if info.has_worldgen {
            println!("  {} World Generation: {}", style("↪").dim(), "Yes");
        }
    }
    println!();
}
