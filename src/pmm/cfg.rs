use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// Represents the configuration for the processes.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
  pub processes: HashMap<String, Process>,
}

/// Represents a process with its command, directory, and shell options.
#[derive(Debug, Clone, Deserialize)]
pub struct Process {
  pub command: String,
  pub directory: Option<PathBuf>,
  pub shell: Option<String>,
}

impl Config {
  /// Loads the configuration from the specified file path.
  ///
  /// # Arguments
  ///
  /// * `path` - The path to the configuration file.
  ///
  /// # Returns
  ///
  /// Returns a `Result` containing the loaded configuration if successful, or an `anyhow::Error` if an error occurs.
  pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
    let content = std::fs::read_to_string(path)?;
    let config = toml::from_str(&content)?;
    Ok(config)
  }
}