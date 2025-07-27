use crate::config::ProjectConfig;
use crate::error::{ProjectError, Result};
use std::path::Path;
use std::process::Command;

pub fn initialize_node_project(project_path: &Path, config: &ProjectConfig) -> Result<()> {
    // Initialize npm project
    let status = Command::new("npm")
        .arg("init")
        .arg("-y")
        .current_dir(project_path)
        .status()
        .map_err(|e| ProjectError::CommandFailed(format!("npm init: {}", e)))?;

    if !status.success() {
        return Err(ProjectError::CommandFailed("npm init failed".to_string()));
    }

    // Install dev dependencies
    install_dev_dependencies(project_path, config)?;

    // Install regular dependencies
    install_dependencies(project_path, config)?;

    Ok(())
}

fn install_dev_dependencies(project_path: &Path, config: &ProjectConfig) -> Result<()> {
    if config.npm_dev_dependencies.is_empty() {
        return Ok(());
    }

    let mut args = vec!["install", "--save-dev"];
    args.extend(config.npm_dev_dependencies.keys().map(|s| s.as_str()));

    let status = Command::new("npm")
        .args(&args)
        .current_dir(project_path)
        .status()
        .map_err(|e| ProjectError::CommandFailed(format!("npm install dev dependencies: {}", e)))?;

    if !status.success() {
        return Err(ProjectError::CommandFailed(
            "npm install dev dependencies failed".to_string(),
        ));
    }

    Ok(())
}

fn install_dependencies(project_path: &Path, config: &ProjectConfig) -> Result<()> {
    if config.npm_dependencies.is_empty() {
        return Ok(());
    }

    let mut args = vec!["install"];
    args.extend(config.npm_dependencies.keys().map(|s| s.as_str()));

    let status = Command::new("npm")
        .args(&args)
        .current_dir(project_path)
        .status()
        .map_err(|e| ProjectError::CommandFailed(format!("npm install dependencies: {}", e)))?;

    if !status.success() {
        return Err(ProjectError::CommandFailed(
            "npm install dependencies failed".to_string(),
        ));
    }

    Ok(())
}
