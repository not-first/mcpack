use crate::pack_formats;
use anyhow::{Context, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::{write::FileOptions, ZipWriter};

pub fn run(command: &crate::cli::Commands) -> Result<()> {
    if let crate::cli::Commands::Pack { path } = command {
        let datapack_path = match path {
            Some(p) => PathBuf::from(p),
            None => std::env::current_dir()?,
        };

        // Verify it's a datapack directory by checking for pack.mcmeta
        let mcmeta_path = datapack_path.join("pack.mcmeta");
        if !mcmeta_path.exists() {
            anyhow::bail!("Not a datapack directory (pack.mcmeta not found)");
        }

        // Read pack.mcmeta to get format version and create zip name
        let mcmeta = fs::read_to_string(&mcmeta_path)
            .with_context(|| format!("Failed to read {}", mcmeta_path.display()))?;
        let mcmeta: Value = serde_json::from_str(&mcmeta)
            .with_context(|| format!("Failed to parse {}", mcmeta_path.display()))?;

        let pack_format = mcmeta
            .get("pack")
            .and_then(|p| p.get("pack_format"))
            .and_then(|f| f.as_u64())
            .context("Invalid pack.mcmeta: missing pack_format")? as u8;

        // Create zip file name from datapack name and Minecraft version
        let datapack_name = datapack_path
            .file_name()
            .context("Invalid datapack path")?
            .to_string_lossy();

        // Determine where to create the zip file
        let output_dir = if path.is_some() {
            // If path was provided, create in current directory
            std::env::current_dir()?
        } else {
            // If no path (running from inside datapack), create in same directory
            datapack_path.clone()
        };

        let zip_path = output_dir.join(format!(
            "{}{}.zip",
            datapack_name,
            pack_formats::get_version_info(pack_format)
                .map(|info| format!("_{}", info.versions.last().unwrap()))
                .unwrap_or_default()
        ));

        // Check if file exists and prompt for overwrite
        if zip_path.exists() {
            let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "File {} already exists. Overwrite?",
                    zip_path.file_name().unwrap().to_string_lossy()
                ))
                .default(false)
                .interact()?;

            if !confirm {
                println!(
                    "{} {}",
                    style("✗").red(),
                    style("Operation cancelled").bold()
                );
                return Ok(());
            }
        }

        // Count total files to process
        let total_files = count_files(&datapack_path)?;
        let pb = ProgressBar::new(total_files);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} files")?
                .progress_chars("#>-"),
        );
        pb.set_message("Creating zip archive...");

        // Create the zip file
        let zip_file = File::create(&zip_path)
            .with_context(|| format!("Failed to create zip file: {}", zip_path.display()))?;
        let mut zip = ZipWriter::new(zip_file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        // Add files to zip with progress
        add_directory_to_zip(&mut zip, &datapack_path, &datapack_path, options, &pb)?;

        zip.finish()?;
        pb.finish_with_message("Archive created successfully!");

        println!(
            "\n{} Created datapack archive: {}",
            style("✓").green(),
            style(zip_path.file_name().unwrap().to_string_lossy()).cyan()
        );
    }

    Ok(())
}

fn count_files(dir_path: &Path) -> Result<u64> {
    let mut count = 0;
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            count += count_files(&path)?;
        } else {
            count += 1;
        }
    }
    Ok(count)
}

fn add_directory_to_zip<'a>(
    zip: &mut ZipWriter<File>,
    base_path: &Path,
    dir_path: &Path,
    options: FileOptions<'a, ()>,
    progress: &ProgressBar,
) -> Result<()> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(base_path)?;

        if path.is_dir() {
            add_directory_to_zip(zip, base_path, &path, options, progress)?;
        } else {
            zip.start_file(relative_path.to_string_lossy().replace('\\', "/"), options)?;
            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            progress.inc(1);
        }
    }

    Ok(())
}
