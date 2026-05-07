use crate::bench::scheduler;
use crate::config;
use crate::error::AppError;
use crate::http::client;
use crate::output;
use crate::source::RequestSource;
use clap::Parser;
use owo_colors::OwoColorize;
use std::time::Duration;

/// Output format for benchmark results.
#[derive(clap::ValueEnum, Clone, Debug, Default)]
pub enum OutputFormat {
    /// Human-readable colored terminal output
    #[default]
    Terminal,
    /// Machine-readable JSON output
    Json,
}

/// Command-line arguments for the benchmark tool.
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "HTTP load benchmarking tool",
    long_about = "Fast, concurrent HTTP load testing with connection pooling.\n\nExamples:\n  blitz config.toml -n 100 -c 10    # 100 requests, 10 concurrent\n  blitz config.toml --format json   # JSON output for scripting\n  blitz config.toml --timeout 30    # 30s timeout per request"
)]
pub struct CliArgs {
    /// Path to .toml config file defining the HTTP request
    request_source: RequestSource,

    /// Total number of requests to send
    #[clap(short = 'n', long, default_value_t = 1)]
    request_count: u64,

    /// Number of concurrent requests (auto-capped at request count)
    #[clap(short, long)]
    concurrency: Option<u64>,

    /// Output format (terminal=colored text, json=machine-readable)
    #[clap(long, value_enum, default_value_t = OutputFormat::Terminal)]
    format: OutputFormat,

    /// Timeout per request in seconds
    #[clap(long, value_parser = parse_timeout)]
    timeout: Option<Duration>,
}

/// Parse a timeout string like "30" or "1.5" into a Duration.
fn parse_timeout(s: &str) -> Result<Duration, String> {
    let secs: f64 = s
        .parse()
        .map_err(|_| format!("'{}' is not a valid timeout in seconds", s))?;
    if secs <= 0.0 {
        return Err("timeout must be greater than 0".to_string());
    }
    Ok(Duration::from_secs_f64(secs))
}

impl CliArgs {
    /// Parse command-line arguments.
    pub fn parse() -> Result<Self, AppError> {
        Ok(CliArgs::try_parse()?)
    }

    /// Get the effective concurrency, with validation.
    /// Returns a warning if concurrency exceeds request count.
    pub fn concurrency(&self) -> u64 {
        let con = self.concurrency.unwrap_or(1);
        if con > self.request_count {
            eprintln!(
                "\n{} concurrency ({}) exceeds request count ({}), capping at {}",
                "Warning:".yellow().bold(),
                con,
                self.request_count,
                self.request_count
            );
            return self.request_count;
        }
        con
    }

    /// Run the benchmark with the parsed arguments.
    /// This is the main orchestration method.
    pub async fn run(&self) -> Result<(), AppError> {
        match &self.request_source {
            RequestSource::Url(_url) => {
                eprintln!(
                    "{} Direct URL benchmarking not yet implemented. Use a .toml config file.",
                    "Error:".red().bold()
                );
                Err(AppError::InvalidSource(
                    "URL mode not yet implemented, use a .toml config file instead".into(),
                ))
            }

            RequestSource::ConfigFile(path) => {
                println!(
                    "{} {:?}",
                    "⚙️  Loading config from".yellow(),
                    path.cyan()
                );

                let cfg = config::load_request_config(path)?;
                cfg.display();

                // Build the shared HTTP client (once, with connection pooling)
                let http_client = client::build_client(self.timeout)?;

                // Get concurrency once (prints warning if needed)
                let concurrency = self.concurrency();

                if concurrency > 1 {
                    println!(
                        "🚀 Sending {} requests with {} concurrency...",
                        self.request_count.to_string().cyan(),
                        concurrency.to_string().cyan(),
                    );
                } else {
                    println!("🚀 Sending request...");
                }

                // Run the benchmark (returns structured BenchmarkOutcome)
                let outcome = scheduler::run_benchmark(
                    cfg,
                    http_client,
                    self.request_count,
                    concurrency,
                )
                .await;

                // Print results in the requested format
                output::print_outcome(&outcome, &self.format);
                Ok(())
            }
        }
    }
}
