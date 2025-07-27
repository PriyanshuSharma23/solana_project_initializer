use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub solana_version: String,
    pub cargo_dependencies: HashMap<String, String>,
    pub npm_dev_dependencies: HashMap<String, String>,
    pub npm_dependencies: HashMap<String, String>,
    pub typescript_config: TypeScriptConfig,
    pub jest_config: JestConfig,
    pub program_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptConfig {
    pub target: String,
    pub module: String,
    pub lib: Vec<String>,
    pub strict: bool,
    pub out_dir: String,
    pub root_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JestConfig {
    pub preset: String,
    pub test_environment: String,
    pub test_match: Vec<String>,
    pub module_file_extensions: Vec<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        let mut cargo_deps = HashMap::new();
        cargo_deps.insert("solana-program".to_string(), "2.3.0".to_string());

        let mut npm_dev_deps = HashMap::new();
        npm_dev_deps.insert("jest".to_string(), "^30.0.5".to_string());
        npm_dev_deps.insert("typescript".to_string(), "^5.8.3".to_string());
        npm_dev_deps.insert("@types/jest".to_string(), "^30.0.0".to_string());
        npm_dev_deps.insert("ts-jest".to_string(), "^29.4.0".to_string());

        let mut npm_deps = HashMap::new();
        npm_deps.insert("@solana/web3.js".to_string(), "^1.98.2".to_string());
        npm_deps.insert("borsh".to_string(), "^2.0.0".to_string());

        Self {
            solana_version: "2.3.0".to_string(),
            cargo_dependencies: cargo_deps,
            npm_dev_dependencies: npm_dev_deps,
            npm_dependencies: npm_deps,
            typescript_config: TypeScriptConfig::default(),
            jest_config: JestConfig::default(),
            program_template: "hello_world".to_string(),
        }
    }
}

impl Default for TypeScriptConfig {
    fn default() -> Self {
        Self {
            target: "ES2020".to_string(),
            module: "commonjs".to_string(),
            lib: vec!["ES2020".to_string(), "DOM".to_string()],
            strict: true,
            out_dir: "./dist".to_string(),
            root_dir: "./".to_string(),
        }
    }
}

impl Default for JestConfig {
    fn default() -> Self {
        Self {
            preset: "ts-jest".to_string(),
            test_environment: "node".to_string(),
            test_match: vec!["**/tests/**/*.test.ts".to_string()],
            module_file_extensions: vec!["ts".to_string(), "js".to_string()],
        }
    }
}
