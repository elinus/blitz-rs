use std::collections::HashMap;
use std::time::Duration;

/// Holds all results from a single benchmark run.
/// This is the single source of truth for benchmark data,
/// replacing the six loose arguments that were passed through scheduler.
#[derive(Clone)]
pub struct BenchmarkOutcome {
    /// Latency of each successful request
    pub latencies: Vec<Duration>,
    /// Total wall-clock time for the entire benchmark
    pub total_time: Duration,
    /// Count of responses grouped by HTTP status code
    pub status_counts: HashMap<u16, u64>,
    /// Number of requests that failed to get a response
    pub errors: u64,
    /// Total number of requests sent (successful + failed)
    pub total: u64,
}

impl BenchmarkOutcome {
    /// Convert latencies to milliseconds and sort them.
    /// Needed for percentile calculations.
    pub fn latencies_ms_sorted(&self) -> Vec<u128> {
        let mut ms: Vec<u128> =
            self.latencies.iter().map(|d| d.as_millis()).collect();
        ms.sort_unstable();
        ms
    }

    /// Calculate a percentile from a sorted latency array.
    /// Returns 0 if no latencies recorded.
    pub fn percentile(&self, sorted: &[u128], p: f64) -> u128 {
        if sorted.is_empty() {
            return 0;
        }
        let idx = ((p / 100.0) * sorted.len() as f64).ceil() as usize;
        sorted[idx.saturating_sub(1).min(sorted.len() - 1)]
    }

    /// Requests per second throughput.
    pub fn req_per_sec(&self) -> f64 {
        if self.total_time.is_zero() {
            return 0.0;
        }
        self.total as f64 / self.total_time.as_secs_f64()
    }

    /// Count of 2xx responses.
    pub fn success_count(&self) -> u64 {
        self.status_counts
            .iter()
            .filter(|(code, _)| (200..300).contains(*code))
            .map(|(_, count)| count)
            .sum()
    }
}
