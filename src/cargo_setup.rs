use crate::config::ProjectConfig;
use crate::error::{ProjectError, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn initialize_cargo_project(project_path: &Path, config: &ProjectConfig) -> Result<()> {
    // Initialize Cargo library
    let status = Command::new("cargo")
        .arg("init")
        .arg("--lib")
        .arg("--vcs")
        .arg("none")
        .arg(project_path)
        .status()
        .map_err(|e| ProjectError::CommandFailed(format!("cargo init: {}", e)))?;

    if !status.success() {
        return Err(ProjectError::CommandFailed("cargo init failed".to_string()));
    }

    // Add dependencies to Cargo.toml
    add_cargo_dependencies(project_path, config)?;

    Ok(())
}

fn add_cargo_dependencies(project_path: &Path, config: &ProjectConfig) -> Result<()> {
    let cargo_toml_path = project_path.join("Cargo.toml");
    let mut cargo_toml = fs::read_to_string(&cargo_toml_path)?;

    // Add cargo-features at the beginning
    if !cargo_toml.contains("cargo-features") {
        cargo_toml = format!("cargo-features = [\"edition2024\"]\n\n{}", cargo_toml);
    }

    // Add lib section if it doesn't exist
    if !cargo_toml.contains("[lib]") {
        // Find the end of [package] section to insert [lib] after it
        if let Some(package_end) = cargo_toml.find("\n[dependencies]") {
            let lib_section = "\n[lib]\ncrate-type = [\"cdylib\", \"lib\"]\n";
            cargo_toml.insert_str(package_end, lib_section);
        } else {
            // If no [dependencies] section, add [lib] before it
            cargo_toml.push_str("\n[lib]\ncrate-type = [\"cdylib\", \"lib\"]\n");
        }
    }

    // Add dependencies section if it doesn't exist
    if !cargo_toml.contains("[dependencies]") {
        cargo_toml.push_str("\n[dependencies]\n");
    }

    // Add each dependency
    for (name, version) in &config.cargo_dependencies {
        let dep_line = format!("{} = \"{}\"\n", name, version);
        if !cargo_toml.contains(&format!("{} =", name)) {
            cargo_toml.push_str(&dep_line);
        }
    }

    fs::write(&cargo_toml_path, cargo_toml)?;
    Ok(())
}
