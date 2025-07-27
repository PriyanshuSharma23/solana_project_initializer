//! The goal of this cli is to produce a solana project with solana_program and a typescript tests/ directory with jest
//! Usage:
//! solanainit ./pdademo
//!
//! This will initialize the project with nodejs and cargo lib, also it should use no vcs by default.

use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let path = std::env::args().nth(1).expect("No path provided");
    let project_path = Path::new(&path);

    // 1. Create project directory
    fs::create_dir_all(&project_path).expect("Failed to create project directory");

    // 2. Initialize Cargo library
    Command::new("cargo")
        .arg("init")
        .arg("--lib")
        .arg("--vcs")
        .arg("none")
        .arg(&path)
        .status()
        .expect("Failed to initialize cargo project");

    // 3. Add solana_program as a dependency
    let cargo_toml_path = project_path.join("Cargo.toml");
    let mut cargo_toml = fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");
    if !cargo_toml.contains("solana-program") {
        cargo_toml.push_str("\nsolana-program = \"2.3.0\"\n");
        fs::write(&cargo_toml_path, cargo_toml).expect("Failed to write Cargo.toml");
    }

    // 4. Add node_modules to .gitignore assuming it always exists
    let gitignore_path = project_path.join(".gitignore");
    let gitignore_content = r#"
target/
node_modules/
dist/
.DS_Store
*.log
*.tsbuildinfo
.env
coverage/
# Solana artifacts
*.so
*.keypair.json"#;

    fs::write(&gitignore_path, gitignore_content).expect("Failed to write .gitignore");

    // 5. Write a hello world Solana program to src/lib.rs
    let lib_rs_path = project_path.join("src/lib.rs");
    let solana_hello = r#"
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, Solana World!");
    Ok(())
}"#;
    fs::write(&lib_rs_path, solana_hello).expect("Failed to write hello world Solana program");

    // 6. Initialize Node.js project in tests/ with Jest and TypeScript
    let tests_path = project_path.join("tests");
    fs::create_dir_all(&tests_path).expect("Failed to create tests directory");

    Command::new("npm")
        .arg("init")
        .arg("-y")
        .current_dir(&project_path)
        .status()
        .expect("Failed to initialize npm project");

    Command::new("npm")
        .arg("install")
        .arg("--save-dev")
        .arg("jest")
        .arg("typescript")
        .arg("@types/jest")
        .arg("ts-jest")
        .current_dir(&project_path)
        .status()
        .expect("Failed to install jest/typescript");

    Command::new("npm")
        .arg("install")
        .arg("@solana/web3.js")
        .arg("borsh")
        .current_dir(&project_path)
        .status()
        .expect("Failed to install jest/typescript");

    // 7. Write tsconfig.json
    let tsconfig = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "lib": ["ES2020", "DOM"],
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true,
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "outDir": "./dist",
    "rootDir": "./"
  },
  "include": ["src/**/*", "tests/**/*"],
  "exclude": ["node_modules", "dist"]
}"#;
    fs::write(project_path.join("tsconfig.json"), tsconfig).expect("Failed to write tsconfig.json");

    // 8. Write jest.config.js
    let jest_config = r#"module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  testMatch: ['**/tests/**/*.test.ts'],
  moduleFileExtensions: ['ts', 'js'],
  transform: {
    '^.+\\.ts$': 'ts-jest',
  },
};"#;
    fs::write(project_path.join("jest.config.js"), jest_config)
        .expect("Failed to write jest.config.js");

    // 9. Create a sample test file
    let sample_test = r#"test('example', () => { expect(1 + 1).toBe(2); });"#;
    fs::write(tests_path.join("example.test.ts"), sample_test)
        .expect("Failed to write sample test");

    println!("Solana project initialized at {}", path);
}
