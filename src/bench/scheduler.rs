use crate::bench::outcome::BenchmarkOutcome;
use crate::bench::worker;
use crate::config::RequestConfig;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::task::JoinSet;

/// Run a benchmark with the given config, client, request count, and concurrency.
/// Returns a structured BenchmarkOutcome with all metrics.
pub async fn run_benchmark(
    cfg: RequestConfig,
    client: reqwest::Client,
    total: u64,
    concurrency: u64,
) -> BenchmarkOutcome {
    let start_all = Instant::now();
    let bar = ProgressBar::new(total);
    let mut set = JoinSet::new();

    let mut latencies: Vec<Duration> = Vec::with_capacity(total as usize);
    let mut status_counts: HashMap<u16, u64> = HashMap::new();
    let mut errors: u64 = 0;
    let mut launched: u64 = 0;

    // Spawn initial wave of workers (min with total so we don't over-spawn)
    for _ in 0..concurrency.min(total) {
        set.spawn(worker::run_one(cfg.clone(), client.clone()));
        launched += 1;
    }

    // Collect results and keep the pipeline full
    while let Some(res) = set.join_next().await {
        bar.inc(1);

        match res {
            Ok((Ok(status), duration)) => {
                latencies.push(duration);
                *status_counts.entry(status).or_insert(0) += 1;
            }
            _ => errors += 1,
        }

        // Launch next worker if we haven't sent all requests yet
        if launched < total {
            set.spawn(worker::run_one(cfg.clone(), client.clone()));
            launched += 1;
        }
    }

    bar.finish_and_clear();

    BenchmarkOutcome {
        latencies,
        total_time: start_all.elapsed(),
        status_counts,
        errors,
        total,
    }
}
