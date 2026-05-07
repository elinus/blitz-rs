use crate::error::AppError;
use std::{path::PathBuf, str::FromStr};

#[derive(Clone, Debug)]
pub enum RequestSource {
    Url(String),
    ConfigFile(PathBuf),
}

impl FromStr for RequestSource {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();

        if input.is_empty() {
            return Err(AppError::EmptySource);
        }

        if input.ends_with(".toml") {
            return Ok(RequestSource::ConfigFile(PathBuf::from(input)));
        }

        if input.starts_with("http://") || input.starts_with("https://") {
            return Ok(RequestSource::Url(input.to_string()));
        }

        Err(AppError::InvalidSource(input.to_string()))
    }
}
