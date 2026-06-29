# port-scanner
A fast, async TCP port scanner built in Rust. Scans a host, grabs service banners, shows a real-time progress bar, and saves results to a file.

> ⚠️ **Educational Project** — This tool was built for learning purposes (networking, Rust, async programming). Only use it against hosts you own or have explicit permission to scan. Unauthorized scanning may violate local laws.

## Features
- Async scanning via Tokio — no thread-per-port overhead
- Banner grabbing on open ports
- Progress bar during scan (`indicatif`)
- Clean CLI interface with flags (`clap`)
- Structured logging (`tracing`)
- Saves results to `results.txt`

## Requirements
- Rust 1.85+ (edition 2024)
- Cargo

## Installation
```bash
git clone https://github.com/DaviAlcanfor/port-scanner.git
cd port-scanner
cargo build --release
```
The binary will be at `./target/release/port-scanner`.

## Usage
```bash
port-scanner [OPTIONS] --host <HOST>
```

### Options
| Flag | Default | Description |
|---|---|---|
| `--host` | required | Target IP or hostname |
| `--ports` | `1-1024` | Port range to scan |
| `--timeout` | `200` | Timeout in ms per port |
| `--threads` | `2` | Concurrent tasks |

### Examples
```bash
# Scan default port range
port-scanner --host 192.168.1.1

# Scan a specific range
port-scanner --host 192.168.1.1 --ports 1-65535

# Custom timeout and threads
port-scanner --host 192.168.1.1 --ports 1-1024 --timeout 500 --threads 10
```

Results are saved to `results.txt` in the current directory.

## Dependencies
| Crate | Purpose |
|---|---|
| `tokio` | Async runtime |
| `futures` | Future combinators |
| `clap` | CLI argument parsing |
| `indicatif` | Progress bar |
| `tracing` + `tracing-subscriber` | Logging |

## License
MIT — see [LICENSE](LICENSE).
