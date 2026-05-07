use crate::bench::outcome::BenchmarkOutcome;
use owo_colors::OwoColorize;

/// Print benchmark results to stdout in human-readable colored format.
pub fn print(outcome: &BenchmarkOutcome) {
    let sorted = outcome.latencies_ms_sorted();

    let p50 = outcome.percentile(&sorted, 50.0);
    let p90 = outcome.percentile(&sorted, 90.0);
    let p99 = outcome.percentile(&sorted, 99.0);
    let max = sorted.last().copied().unwrap_or(0);

    println!("\n{}", "Latency".yellow());
    println!(
        "p50 {} p90 {} p99 {} max {}",
        format!("{}ms", p50).cyan(),
        format!("{}ms", p90).cyan(),
        format!("{}ms", p99).cyan(),
        format!("{}ms", max).cyan(),
    );

    println!("\n{}", "Throughput".yellow());
    println!("{:.0} req/sec", outcome.req_per_sec().to_string().green());

    println!("\n{}", "Status codes".yellow());
    let mut codes: Vec<(&u16, &u64)> = outcome.status_counts.iter().collect();
    codes.sort_by_key(|(code, _)| *code);
    for (code, count) in codes {
        println!("  HTTP {}: {}", code, count);
    }

    if outcome.errors > 0 {
        println!("  Errors: {}", outcome.errors.to_string().red());
    }

    println!(
        "\nTotal: {} requests in {:.2}s",
        outcome.total,
        outcome.total_time.as_secs_f64()
    );
}
