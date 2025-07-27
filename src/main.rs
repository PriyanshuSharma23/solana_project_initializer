//! The goal of this cli is to produce a solana project with solana_program and a typescript tests/ directory with jest
//! Usage:
//! solanainit ./pdademo
//!
//! This will initialize the project with nodejs and cargo lib, also it should use no vcs by default.

use std::fs;
use std::path::{Path, PathBuf};

mod cargo_setup;
mod config;
mod error;
mod node_setup;
mod templates;

use config::ProjectConfig;
use error::{ProjectError, Result};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err(ProjectError::Usage(
            "Usage: solanainit <project_path> [--config <config_file>]".to_string(),
        ));
    }

    let project_path = Path::new(&args[1]);

    // Load configuration (can be extended to read from file)
    let config = ProjectConfig::default();

    // Validate project path
    if project_path.exists() && !project_path.read_dir()?.next().is_none() {
        return Err(ProjectError::DirectoryExists(project_path.to_path_buf()));
    }

    println!("Initializing Solana project at: {}", project_path.display());

    // Create project structure
    let project = SolanaProject::new(project_path.to_path_buf(), config)?;
    project.initialize()?;

    println!(
        "✅ Solana project initialized successfully at {}",
        project_path.display()
    );
    println!("📁 Project structure created");
    println!("🔧 Cargo.toml configured with Solana dependencies");
    println!("🧪 TypeScript test environment set up with Jest");
    println!("📝 Sample Solana program and tests created");

    Ok(())
}

struct SolanaProject {
    path: PathBuf,
    config: ProjectConfig,
}

impl SolanaProject {
    fn new(path: PathBuf, config: ProjectConfig) -> Result<Self> {
        Ok(Self { path, config })
    }

    fn initialize(&self) -> Result<()> {
        // Create project directory
        fs::create_dir_all(&self.path)?;

        // Initialize Cargo project
        self.setup_cargo_project()?;

        // Setup Node.js environment
        self.setup_node_project()?;

        // Create project files
        self.create_project_files()?;

        Ok(())
    }

    fn setup_cargo_project(&self) -> Result<()> {
        cargo_setup::initialize_cargo_project(&self.path, &self.config)?;
        Ok(())
    }

    fn setup_node_project(&self) -> Result<()> {
        node_setup::initialize_node_project(&self.path, &self.config)?;
        Ok(())
    }

    fn create_project_files(&self) -> Result<()> {
        // Create .gitignore
        self.create_gitignore()?;

        // Create Solana program
        self.create_solana_program()?;

        // Create TypeScript configuration
        self.create_typescript_config()?;

        // Create Jest configuration
        self.create_jest_config()?;

        // Create sample test
        self.create_sample_test()?;

        Ok(())
    }

    fn create_gitignore(&self) -> Result<()> {
        let gitignore_path = self.path.join(".gitignore");
        let content = templates::gitignore_template();
        fs::write(gitignore_path, content)?;
        Ok(())
    }

    fn create_solana_program(&self) -> Result<()> {
        let lib_rs_path = self.path.join("src/lib.rs");
        let content = templates::solana_program_template(&self.config);
        fs::write(lib_rs_path, content)?;
        Ok(())
    }

    fn create_typescript_config(&self) -> Result<()> {
        let tsconfig_path = self.path.join("tsconfig.json");
        let content = templates::tsconfig_template(&self.config);
        fs::write(tsconfig_path, content)?;
        Ok(())
    }

    fn create_jest_config(&self) -> Result<()> {
        let jest_config_path = self.path.join("jest.config.js");
        let content = templates::jest_config_template(&self.config);
        fs::write(jest_config_path, content)?;
        Ok(())
    }

    fn create_sample_test(&self) -> Result<()> {
        let tests_path = self.path.join("tests");
        fs::create_dir_all(&tests_path)?;

        let sample_test_path = tests_path.join("example.test.ts");
        let content = templates::sample_test_template(&self.config);
        fs::write(sample_test_path, content)?;
        Ok(())
    }
}
