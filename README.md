# Solana Project Initializer

A flexible and well-structured CLI tool for initializing Solana projects with TypeScript testing environment.

## Features

- **Modular Architecture**: Clean separation of concerns with dedicated modules
- **Configurable Dependencies**: Easy to customize Cargo and npm dependencies
- **Multiple Program Templates**: Support for different Solana program templates
- **Flexible Configuration**: JSON-based configuration system
- **Better Error Handling**: Comprehensive error types and messages
- **Validation**: Input validation and safety checks
- **Extensible**: Easy to add new templates and configurations

## Usage

```bash
# Basic usage
solanainit ./my-solana-project

# With custom configuration (future feature)
solanainit ./my-solana-project --config config.json
```

## Project Structure

The tool creates a complete Solana project with the following structure:

```
my-solana-project/
├── Cargo.toml          # Rust project with Solana dependencies
├── src/
│   └── lib.rs          # Solana program entry point
├── tests/
│   └── example.test.ts # TypeScript test file
├── package.json        # Node.js project
├── tsconfig.json       # TypeScript configuration
├── jest.config.js      # Jest testing configuration
└── .gitignore          # Git ignore file
```

## Configuration

The tool uses a `ProjectConfig` struct that can be customized:

```rust
pub struct ProjectConfig {
    pub solana_version: String,
    pub cargo_dependencies: HashMap<String, String>,
    pub npm_dev_dependencies: HashMap<String, String>,
    pub npm_dependencies: HashMap<String, String>,
    pub typescript_config: TypeScriptConfig,
    pub jest_config: JestConfig,
    pub program_template: String,
}
```

### Default Configuration

- **Solana Program**: 2.3.0
- **TypeScript**: ES2020 target with strict mode
- **Jest**: ts-jest preset for TypeScript testing
- **Program Template**: Hello World example

## Program Templates

Currently supported templates:

1. **hello_world**: Simple "Hello, Solana World!" program
2. **counter**: Basic counter program with account validation

## Architecture

### Modules

- **`main.rs`**: Entry point and project orchestration
- **`config.rs`**: Configuration management
- **`error.rs`**: Custom error types and handling
- **`cargo_setup.rs`**: Cargo project initialization
- **`node_setup.rs`**: Node.js and npm setup
- **`templates.rs`**: File templates and content generation

### Error Handling

The tool uses custom error types for better error messages:

- `ProjectError::Io`: File system errors
- `ProjectError::CommandFailed`: External command failures
- `ProjectError::DirectoryExists`: Directory validation
- `ProjectError::Usage`: Command line usage errors
- `ProjectError::ConfigError`: Configuration errors

## Development

### Building

```bash
cargo build
```

### Running

```bash
cargo run -- ./test-project
```

### Testing

```bash
cargo test
```

## Future Enhancements

- [ ] Configuration file support (JSON/YAML)
- [ ] More program templates (Token, NFT, etc.)
- [ ] CLI argument parsing with clap
- [ ] Interactive mode for configuration
- [ ] Template customization options
- [ ] Integration with Solana CLI tools
- [ ] Support for different testing frameworks
- [ ] Project scaffolding with common patterns

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License
