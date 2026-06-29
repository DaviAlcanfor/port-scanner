# port-scanner

A fast, async TCP port scanner built in Rust. Scans one or multiple hosts, shows a real-time progress bar, and prints results with colored output.

> ⚠️ **Educational Project** — This tool was built for learning purposes (networking, Rust, async programming). Only use it against hosts you own or have explicit permission to scan. Unauthorized scanning may violate local laws.


## Features

- Async scanning via Tokio — no thread-per-port overhead
- ICMP ping check before scanning (`surge-ping`)
- Progress bar during scan (`indicatif`)
- Colored output for open/closed ports (`colored`)
- Clean CLI interface with flags (`clap`)
- Structured logging (`tracing`)

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

### Examples

```bash
# Scan default ports on a host
port-scanner --host 192.168.1.1

# Scan a specific port range
port-scanner --host 192.168.1.1 --start 1 --end 1024

# Scan a single port
port-scanner --host scanme.nmap.org --start 80 --end 80
```

> **Note:** ICMP ping requires elevated privileges on some systems. Run with `sudo` if the ping step fails.

## Dependencies

| Crate | Purpose |
|---|---|
| `tokio` | Async runtime |
| `futures` | Future combinators |
| `clap` | CLI argument parsing |
| `surge-ping` | ICMP ping |
| `indicatif` | Progress bar |
| `colored` | Terminal color output |
| `tracing` + `tracing-subscriber` | Logging |

## License

MIT — see [LICENSE](LICENSE).
