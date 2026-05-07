# Blitz — Fast HTTP Load Benchmarking

A fast, concurrent HTTP load testing tool with connection pooling and machine-readable output.

## Features

- **Concurrent load testing** — spawn multiple simultaneous requests with `-c` flag
- **Connection pooling** — reuses TCP connections for accurate server performance measurement
- **Flexible configuration** — define complex requests in TOML config files
- **Multiple output formats** — terminal (colored) or JSON for scripting
- **Per-request timeout** — `--timeout` flag prevents hanging on slow endpoints
- **Real latency percentiles** — p50, p90, p99, and max latency in milliseconds
- **Status code tracking** — groups responses by HTTP status code
- **Zero external dependencies** — simple binary, no database or external services required

## Installation

From crates.io:

```bash
cargo install blitz
```

From source:

```bash
git clone https://github.com/yourusername/blitz.git
cd blitz
cargo build --release
./target/release/blitz --version
```

## Quick Start

### 1. Create a config file (`benchmark.toml`)

```toml
url = "https://api.example.com/users"
method = "GET"

[headers]
User-Agent = "blitz/0.1.0"
Accept = "application/json"
```

### 2. Run a benchmark

```bash
blitz benchmark.toml -n 100 -c 5
```

This sends 100 requests with 5 concurrent workers.

**Output:**
```
┌─ Request Configuration ─────────────────────┐
│ URL
│   https://api.example.com/users
│ Method
│   GET
│ Headers
│   User-Agent: blitz/0.1.0
│   Accept: application/json
└─────────────────────────────────────────────┘

Running 100 requests...
🚀 Sending requests with 5 concurrency

Latency
p50 42ms p90 87ms p99 203ms max 891ms

Throughput
1243 req/sec

Status codes
  HTTP 200: 98
  HTTP 404: 2

Total: 100 requests in 0.08s
```

## Usage

### Basic syntax

```bash
blitz <CONFIG.toml> [OPTIONS]
```

### Options

```
-n, --request-count <N>      Total requests to send [default: 1]
-c, --concurrency <N>        Concurrent requests [default: 1]
--format <FORMAT>            Output: terminal or json [default: terminal]
--timeout <SECONDS>          Timeout per request [default: no timeout]
-h, --help                   Show help
-V, --version                Show version
```

### Examples

#### Simple GET with default settings
```bash
blitz api.toml
```

#### 1000 requests with 20 concurrent
```bash
blitz api.toml -n 1000 -c 20
```

#### With 30 second timeout per request
```bash
blitz api.toml -n 500 --timeout 30
```

#### JSON output for scripting
```bash
blitz api.toml -n 100 --format json | jq '.latency_ms.p99'
```

## Configuration Format

Create a `.toml` file defining your HTTP request:

### Simple GET
```toml
url = "https://api.example.com/data"
method = "GET"
```

### POST with headers and body
```toml
url = "https://api.example.com/users"
method = "POST"

[headers]
Content-Type = "application/json"
Authorization = "Bearer YOUR_TOKEN"

body = '{"name": "John", "email": "john@example.com"}'
```

### With multiple headers
```toml
url = "https://api.example.com/search"
method = "GET"

[headers]
User-Agent = "blitz/0.1.0"
Accept = "application/json"
Accept-Language = "en-US"
```

## Output Formats

### Terminal (default)

Human-readable colored output:

```
Latency
p50 42ms p90 87ms p99 203ms max 891ms

Throughput
1243 req/sec

Status codes
  HTTP 200: 980
  HTTP 404: 12
  Errors: 8

Total: 1000 requests in 0.80s
```

### JSON (for automation)

```bash
blitz config.toml --format json
```

```json
{
  "total_requests": 1000,
  "successful_requests": 980,
  "total_time_sec": 0.8,
  "req_per_sec": 1250.0,
  "latency_ms": {
    "p50": 42,
    "p90": 87,
    "p99": 203,
    "max": 891
  },
  "status_codes": {
    "200": 980,
    "404": 12
  },
  "errors": 8
}
```

Extract specific metrics:

```bash
# Get p99 latency
blitz config.toml -n 100 --format json | jq '.latency_ms.p99'

# Get requests per second
blitz config.toml -n 100 --format json | jq '.req_per_sec'

# Check error rate
blitz config.toml -n 100 --format json | jq '.errors'
```

## Performance Tips

### Connection pooling
Blitz reuses TCP connections by default. First few requests may be slower (TCP handshake + TLS), then subsequent requests are much faster as connections are reused from the pool.

### Concurrency tuning
- Start with `-c` equal to your CPU cores
- Increase if you see CPU utilization below 80%
- Decrease if you're rate-limited or getting connection errors

### Realistic targets
- Test against staging/production-like environments
- Include realistic request bodies and headers
- Account for network latency in your measurements

## Common Workflows

### Load test and save results
```bash
blitz api.toml -n 5000 -c 20 --format json > results.json
cat results.json | jq .
```

### CI/CD integration — fail if latency regresses
```bash
RESULT=$(blitz api.toml -n 1000 -c 10 --format json)
P99=$(echo $RESULT | jq '.latency_ms.p99')

if (( $(echo "$P99 > 200" | bc -l) )); then
  echo "P99 latency regression detected: ${P99}ms"
  exit 1
fi
```

### Compare two endpoints
```bash
echo "Testing endpoint A:"
blitz a.toml -n 500 -c 5

echo "Testing endpoint B:"
blitz b.toml -n 500 -c 5
```

## Limitations

- HTTP/2 not yet supported (uses HTTP/1.1)
- No TLS certificate verification bypass (always verifies)
- Request body must be plain text (not binary)
- No response body validation (just measures latency and status)

## Building from source

Requirements:
- Rust 1.70+
- Cargo

```bash
git clone https://github.com/yourusername/blitz.git
cd blitz
cargo build --release
./target/release/blitz --help
```

## Contributing

Contributions welcome! Please open an issue or pull request on GitHub.