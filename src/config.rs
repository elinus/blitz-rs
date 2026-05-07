use crate::error::AppError;
use owo_colors::OwoColorize;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// HTTP method enum that deserializes from TOML and converts to reqwest::Method.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::PATCH => "PATCH",
            Self::HEAD => "HEAD",
        };
        write!(f, "{}", s.purple().bold())
    }
}

/// Configuration for a single HTTP request, loaded from TOML.
#[derive(Debug, Deserialize, Clone)]
pub struct RequestConfig {
    /// Target URL
    pub url: String,
    /// HTTP method
    pub method: HttpMethod,
    /// Optional headers (key-value pairs)
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// Optional request body as string
    pub body: Option<String>,
}

impl RequestConfig {
    /// Print the configuration to stdout in a human-readable format.
    /// Display the configuration in a formatted, visually appealing way.
    pub fn display(&self) {
        println!(
            "\n{}",
            "┌─ Request Configuration ─────────────────────┐".dimmed()
        );

        print_section("URL");
        println!("│   {}", truncate(&self.url, 60).green());

        print_section("Method");
        println!("│   {}", self.method);

        if !self.headers.is_empty() {
            print_section("Headers");

            for (k, v) in &self.headers {
                println!("│   {}: {}", k.blue(), truncate(v, 50).dimmed());
            }
        }

        if let Some(body) = &self.body {
            print_section("Body");

            let formatted = format_body(body);

            print_lines(&formatted, 10);
        }

        println!(
            "{}",
            "└─────────────────────────────────────────────┘".dimmed()
        );
        println!();
    }
}

/// Load a RequestConfig from a TOML file.
pub fn load_request_config(path: &Path) -> Result<RequestConfig, AppError> {
    let content = fs::read_to_string(path).map_err(|e| AppError::Io {
        source: e,
        path: path.to_path_buf(),
    })?;

    toml::from_str(&content).map_err(AppError::from)
}

// {{ --- Helper functions for printing benchmark.toml ---
fn print_section(title: &str) {
    println!("│ {}", title.cyan().bold());
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max - 3])
    } else {
        s.to_string()
    }
}

fn format_body(body: &str) -> String {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        serde_json::to_string_pretty(&json).unwrap_or_else(|_| body.to_string())
    } else {
        body.to_string()
    }
}

fn print_lines(content: &str, max_lines: usize) {
    let lines: Vec<&str> = content.lines().collect();

    for line in lines.iter().take(max_lines) {
        println!("│   {}", line.dimmed());
    }

    if lines.len() > max_lines {
        println!(
            "│   {}",
            format!("… ({} more lines)", lines.len() - max_lines).dimmed()
        );
    }
}

// --- Helper functions for printing benchmark.toml }} ---
