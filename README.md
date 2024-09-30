# monotonic-time-rs

[![Crates.io](https://img.shields.io/crates/v/monotonic-time-rs.svg)](https://crates.io/crates/monotonic-time-rs)
[![Documentation](https://docs.rs/monotonic-time-rs/badge.svg)](https://docs.rs/monotonic-time-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`monotonic-time-rs` is a Rust library dedicated to managing monotonic absolute timestamps with millisecond precision. It is primarily utilized for measuring latency and serializing monotonic time data for efficient transmission over networks.

## 🚀 Features

- **Monotonic Timestamps**: Represent absolute time points in milliseconds.
- **Durations**: Safely compute elapsed time between timestamps.
- **Lower Bits Extraction**: Efficiently handle partial timestamp data.
- **Custom Time Providers**: Implement your own monotonic clock sources.

## 📦 Installation

Add `monotonic-time-rs` to your `Cargo.toml`:

```toml
[dependencies]
monotonic-time-rs = "^0.0.1"
```
