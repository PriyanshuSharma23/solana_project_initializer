use crate::config::ProjectConfig;

pub fn gitignore_template() -> String {
    r#"
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
*.keypair.json
# IDE files
.vscode/
.idea/
*.swp
*.swo
"#
    .trim()
    .to_string()
}

pub fn solana_program_template(config: &ProjectConfig) -> String {
    match config.program_template.as_str() {
        "hello_world" => hello_world_template(),
        "counter" => counter_template(),
        _ => hello_world_template(),
    }
}

fn hello_world_template() -> String {
    r#"
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
}
"#
    .trim()
    .to_string()
}

fn counter_template() -> String {
    r#"
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let counter_account = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let rent = &Rent::from_account_info(next_account_info(accounts_iter)?)?;

    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Counter program initialized");
    Ok(())
}
"#
    .trim()
    .to_string()
}

pub fn tsconfig_template(config: &ProjectConfig) -> String {
    let lib_json = serde_json::to_string(&config.typescript_config.lib).unwrap();

    format!(
        r#"{{
  "compilerOptions": {{
    "target": "{}",
    "module": "{}",
    "lib": {},
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true,
    "strict": {},
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "outDir": "{}",
    "rootDir": "{}"
  }},
  "include": ["src/**/*", "tests/**/*"],
  "exclude": ["node_modules", "dist"]
}}"#,
        config.typescript_config.target,
        config.typescript_config.module,
        lib_json,
        config.typescript_config.strict,
        config.typescript_config.out_dir,
        config.typescript_config.root_dir
    )
}

pub fn jest_config_template(config: &ProjectConfig) -> String {
    let test_match_json = serde_json::to_string(&config.jest_config.test_match).unwrap();
    let module_file_extensions_json =
        serde_json::to_string(&config.jest_config.module_file_extensions).unwrap();

    format!(
        r#"module.exports = {{
  preset: '{}',
  testEnvironment: '{}',
  testMatch: {},
  moduleFileExtensions: {},
  transform: {{
    '^.+\\\\.ts$': 'ts-jest',
  }},
}};"#,
        config.jest_config.preset,
        config.jest_config.test_environment,
        test_match_json,
        module_file_extensions_json
    )
}

pub fn sample_test_template(_config: &ProjectConfig) -> String {
    r#"import { Connection, PublicKey } from '@solana/web3.js';

describe('Solana Program Tests', () => {
  let connection: Connection;

  beforeAll(() => {
    connection = new Connection('http://localhost:8899', 'confirmed');
  });

  test('example test', () => {
    expect(1 + 1).toBe(2);
  });

  test('connection test', async () => {
    const version = await connection.getVersion();
    expect(version).toBeDefined();
  });
});"#
        .trim()
        .to_string()
}
