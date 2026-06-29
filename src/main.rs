mod cli;
use cli::Args;
use clap::Parser;
use std::fs;

mod probe;
use probe::probe;

use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::time::Duration;
use tracing::error;

fn init_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
    pb.set_message("Scanning ports...");
    pb
}

fn init_logger() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();
}

#[tokio::main]
async fn main() {
    init_logger();
    let pb = Arc::new(init_progress_bar());
    let args = Args::parse();
    let threads = args.threads;
    let timeout_ms = args.timeout;
    let host = Arc::new(args.host.clone());

    let ports = match args.map_ports() {
        Some(p) => p,
        None => {
            error!("Invalid port range. Use format 1-1024");
            return;
        }
    };

    let results: Vec<(u16, String)> = stream::iter(ports)
        .map(|port| {
            let host = host.clone();
            let pb = pb.clone();
            async move {
                let banner = probe(&host, port, timeout_ms).await;
                if banner.is_some() {
                    pb.println(format!("OPEN {}", port));
                }
                (port, banner)
            }
        })
        .buffer_unordered(threads as usize)
        .filter_map(|(port, banner)| async move {
            banner.map(|b| (port, b))
        })
        .collect()
        .await;

    let output = results
        .iter()
        .map(|(port, banner)| {
            if banner.is_empty() {
                format!("OPEN {}", port)
            } else {
                format!("OPEN {} | {}", port, banner.trim())
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    pb.finish_with_message("Done.");
    fs::write("results.txt", output).expect("Failed to write results file");
}