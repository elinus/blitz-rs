pub mod json;
pub mod summary;
pub mod terminal;

use crate::bench::outcome::BenchmarkOutcome;
use crate::cli::OutputFormat;

/// Print benchmark outcome in the requested format (terminal or JSON).
pub fn print_outcome(outcome: &BenchmarkOutcome, format: &OutputFormat) {
    match format {
        OutputFormat::Terminal => summary::print(outcome),
        OutputFormat::Json => json::print(outcome),
    }
}
