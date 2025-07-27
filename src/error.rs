use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ProjectError {
    Io(io::Error),
    CommandFailed(String),
    DirectoryExists(PathBuf),
    Usage(String),
    ConfigError(String),
}

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectError::Io(err) => write!(f, "IO error: {}", err),
            ProjectError::CommandFailed(cmd) => write!(f, "Command failed: {}", cmd),
            ProjectError::DirectoryExists(path) => write!(
                f,
                "Directory already exists and is not empty: {}",
                path.display()
            ),
            ProjectError::Usage(msg) => write!(f, "Usage error: {}", msg),
            ProjectError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for ProjectError {}

impl From<io::Error> for ProjectError {
    fn from(err: io::Error) -> Self {
        ProjectError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, ProjectError>;
