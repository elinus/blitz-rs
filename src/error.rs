use std::path::PathBuf;
use thiserror::Error;

/// Unified error type for all application errors.
#[derive(Error, Debug)]
pub enum AppError {
    // --- HTTP & Network ---
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Failed to build HTTP client: {0}")]
    HttpClient(reqwest::Error),

    // --- System Errors ---
    #[error("Failed to read config file at {path}: {source}")]
    Io {
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("Config parsing failed: {0}")]
    Parse(#[from] toml::de::Error),

    // --- User Input Errors ---
    #[error("CLI Argument error: {0}")]
    Cli(#[from] clap::Error),

    #[error("Request source cannot be empty")]
    EmptySource,

    #[error("Invalid request source: '{0}'")]
    InvalidSource(String),
}

impl AppError {
    /// Return a user-friendly help message for this error.
    pub fn help_message(&self) -> &str {
        match self {
            Self::EmptySource | Self::InvalidSource(_) => {
                "Provide a full URL (https://api.example.com) or a path to a .toml file."
            }
            Self::Cli(_) => {
                "Check the usage above. Try: benchmark config.toml or benchmark https://example.com"
            }
            Self::Io { .. } | Self::Parse(_) => {
                "Ensure the file exists and is a valid TOML configuration."
            }
            Self::Http(_) | Self::HttpClient(_) => {
                "Check the URL is reachable and the request is valid."
            }
        }
    }
}
