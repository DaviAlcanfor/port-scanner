use clap::Parser;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use futures::stream::{self, StreamExt};
use std::sync::Arc;
use tracing::error;
use indicatif::{ProgressBar, ProgressStyle};




#[derive(Parser, Debug)]
#[command(version, about, about = "Fast TCP port scanner made in Rust")]
struct Args {
    
    #[arg(long, help = "Target IP or hostname")]
    host: String,
    
    #[arg(long, default_value_t = 200, help = "Timeout in ms between ports")]
    timeout: u64,

    #[arg(long, default_value_t = 1, help = "Number of concurrent tasks by thread")]
    threads: u16,

    #[arg(long, default_value = "1-1024", help = "Range of ports")]
    ports: String,
}



fn map_ports(ports_str: String) -> Option<Vec<u16>> {
    let (start, end) = ports_str.split_once("-")?;
    
    let start: u16 = start.parse().ok()?;
    let end: u16 = end.parse().ok()?;
    
    Some((start..=end).collect())
}


async fn probe(host: &str, port: u16, timeout_ms: u64) -> bool {
    let addr = format!("{}:{}", host, port);
    let duration = Duration::from_millis(timeout_ms);

    let result = timeout(duration, TcpStream::connect(&addr)).await;
    
    match result{
        Ok(Ok(_)) => true,
        _ => false
    }
}


#[tokio::main]
async fn main() {
    
    // Basic log for error handling 
    tracing_subscriber::fmt()
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();


    let pb = Arc::new(ProgressBar::new_spinner());
    let args = Args::parse();
    let host = Arc::new(args.host);
    let timeout_ms = args.timeout;
    
    // Log for progress
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
        .unwrap(),
    );
    pb.set_message("Scanning ports...");



    let ports = match map_ports(args.ports) {
        Some(p) => p,
        None => {
            error!("Invalid port range. Use format 1-1024");
            return;
        }
    };


    stream::iter(ports)
        .for_each_concurrent(args.threads as usize, |port| {
            let host = host.clone();
            let value = pb.clone();
            async move {
                if probe(&host, port, timeout_ms).await {
                    value.println(format!("OPEN {}", port));
                }

            }
        })
        .await;
}
