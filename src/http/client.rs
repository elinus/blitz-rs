use crate::error::AppError;
use std::time::Duration;

/// Build a single shared reqwest::Client with connection pooling.
/// All requests will share this client instance and reuse connections to the same hosts.
///
/// # Arguments
/// * `timeout` - Optional timeout for individual requests. If not set, no timeout is applied.
///
/// # Returns
/// A configured Client ready for use, or an AppError if construction fails.
pub fn build_client(
    timeout: Option<Duration>,
) -> Result<reqwest::Client, AppError> {
    let mut builder = reqwest::Client::builder()
        .pool_max_idle_per_host(100) // keep up to 100 idle connections per host
        .tcp_keepalive(Duration::from_secs(30)); // send TCP keep-alive every 30s

    if let Some(t) = timeout {
        builder = builder.timeout(t);
    }

    builder.build().map_err(AppError::HttpClient)
}
