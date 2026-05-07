use crate::config::RequestConfig;
use crate::error::AppError;
use reqwest::Method;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

/// Convert our internal HttpMethod enum to reqwest::Method.
fn method_to_reqwest(method: &crate::config::HttpMethod) -> Method {
    match method {
        crate::config::HttpMethod::GET => Method::GET,
        crate::config::HttpMethod::POST => Method::POST,
        crate::config::HttpMethod::PUT => Method::PUT,
        crate::config::HttpMethod::DELETE => Method::DELETE,
        crate::config::HttpMethod::PATCH => Method::PATCH,
        crate::config::HttpMethod::HEAD => Method::HEAD,
    }
}

/// Execute a single HTTP request using the shared client.
///
/// # Arguments
/// * `cfg` - The request configuration (URL, method, headers, body)
/// * `client` - The shared reqwest::Client (owns the connection pool)
///
/// # Returns
/// A reqwest::Response on success, or AppError on failure.
pub async fn execute(
    cfg: &RequestConfig,
    client: &reqwest::Client,
) -> Result<reqwest::Response, AppError> {
    let method = method_to_reqwest(&cfg.method);

    let mut headers = HeaderMap::new();
    for (k, v) in &cfg.headers {
        if let (Ok(name), Ok(value)) =
            (HeaderName::from_str(k), HeaderValue::from_str(v))
        {
            headers.insert(name, value);
        }
    }

    let mut req = client.request(method, &cfg.url).headers(headers);

    if let Some(body) = &cfg.body {
        req = req.body(body.clone());
    }

    req.send().await.map_err(AppError::Http)
}
