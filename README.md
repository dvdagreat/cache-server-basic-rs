
## Overview

A blazingly fast, distributed caching server built with Rust. Designed for high-performance data access with a focus on reliability and scalability.

## Quick Demo

<img src="./assets/demo.gif" alt="Cache server demo" width="800">

## Key Features

- Thread-safe in-memory storage for lightning-fast access
- Powered by Tokio for non-blocking async operations
- Simple, serializable protocol that's easy to work with
- Modular workspace structure that keeps things organized
- Supports ttl (time-to-live) for cache entries (default: 60 seconds)

## What You Can Do

- **GET**: Fetch values by their keys
- **SET**: Save your key-value pairs
- **HAS**: Check whether a key is in the cache
- **STATS**: See what's happening under the hood
- **QUIT**: Close your connection

## How It's Organized

- **cache_protocol**: Handles serialization between client and server
- **cache_server**: The core server with all the networking logic

## What's Under the Hood

- `tokio` — async runtime that keeps things moving
- `serde`/`serde_json` — data serialization made simple
- `dashmap` — concurrent hash map for thread safety
- `bytes` — efficient byte operations
- `tracing` — logging that actually tells you what's happening
- `chrono` — timestamp handling

## Quick Start

Build it:
```bash
cargo build -p cache_server --release
```

Run it:
```bash
cargo run -p cache_server --release
```

Invoke using `netcat`:
```bash
nc localhost 8080
```

or for single invocation:

```bash
echo "SET mykey myvalue 60" | nc -N localhost 8080
echo "GET mykey" | nc -N localhost 8080
```
