use crate::bench::outcome::BenchmarkOutcome;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct LatencyMs {
    p50: u128,
    p90: u128,
    p99: u128,
    max: u128,
}

#[derive(Serialize)]
struct JsonOutput {
    total_requests: u64,
    successful_requests: u64,
    total_time_sec: f64,
    req_per_sec: f64,
    latency_ms: LatencyMs,
    status_codes: HashMap<u16, u64>,
    errors: u64,
}

/// Print benchmark results as JSON to stdout.
/// The output is pretty-printed if stdout is a terminal,
/// compact if piped.
pub fn print(outcome: &BenchmarkOutcome) {
    let sorted = outcome.latencies_ms_sorted();

    let output = JsonOutput {
        total_requests: outcome.total,
        successful_requests: outcome.success_count(),
        total_time_sec: outcome.total_time.as_secs_f64(),
        req_per_sec: outcome.req_per_sec(),
        latency_ms: LatencyMs {
            p50: outcome.percentile(&sorted, 50.0),
            p90: outcome.percentile(&sorted, 90.0),
            p99: outcome.percentile(&sorted, 99.0),
            max: sorted.last().copied().unwrap_or(0),
        },
        status_codes: outcome.status_counts.clone(),
        errors: outcome.errors,
    };

    // pretty print if interactive, compact if piped
    let json = if std::io::IsTerminal::is_terminal(&std::io::stdout()) {
        serde_json::to_string_pretty(&output)
    } else {
        serde_json::to_string(&output)
    };

    match json {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("Failed to serialize JSON output: {}", e),
    }
}
