# Rust Port Scanner

A lightweight, multi-threaded port scanner implemented in Rust.

## Features

- **High-performance** scanning using multiple threads
- Scans all **65,535 ports** by default
- Real-time progress indicators
- Configurable thread count

## Example

```bash
./target/release/port-scanner -j 10 127.0.0.1
