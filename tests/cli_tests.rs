use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

/*
CLI Integration Tests

This file tests the CLI interface directly through command-line invocations.
Tests should cover:

1. Command Validation
   - Invalid commands
   - Missing required arguments
   - Invalid argument combinations
   - Help text display

2. Create Command
   - Basic datapack creation
   - Custom namespace creation
   - Icon handling
   - Pack format validation
   - Multiple pack formats
   - Minecraft namespace options
   - Force overwrite behavior
   - Output directory specification

3. Info Command
   - Basic info display
   - Compact mode
   - Pack info mode
   - Namespaces mode
   - ZIP file info
   - Invalid datapack handling

4. Zip Command
   - Basic zip creation
   - Custom name handling
   - Output directory specification
   - Force overwrite behavior
   - Input directory handling

5. Add Command
   - Element type validation
   - Namespace handling
   - File naming
   - Force overwrite behavior
   - Template content verification
*/

#[test]
fn test_invalid_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mcpack")?;
    cmd.arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
    Ok(())
}

#[test]
fn test_create_basic_datapack() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;
    let mut cmd = Command::cargo_bin("mcpack")?;

    cmd.arg("create")
        .arg("--name")
        .arg("test-pack")
        .arg("--description")
        .arg("Test datapack")
        .arg("--format")
        .arg("48")
        .arg("--no-icon")
        .arg("--no-starter-files")
        .arg("--output-dir")
        .arg(temp.path());

    cmd.assert().success();

    temp.child("test-pack/pack.mcmeta")
        .assert(predicate::path::exists());
    temp.child("test-pack/data")
        .assert(predicate::path::exists());

    Ok(())
}
