use crate::config::RequestConfig;
use crate::http::executor;
use std::time::{Duration, Instant};

/// Run a single HTTP request, returning (status or error, duration).
/// The client is Arc'd internally, so clone is cheap and all workers
/// share the same connection pool.
pub async fn run_one(
    cfg: RequestConfig,
    client: reqwest::Client,
) -> (Result<u16, String>, Duration) {
    let start = Instant::now();
    let result = executor::execute(&cfg, &client).await;
    let duration = start.elapsed();

    match result {
        Ok(resp) => (Ok(resp.status().as_u16()), duration),
        Err(e) => (Err(e.to_string()), duration),
    }
}
