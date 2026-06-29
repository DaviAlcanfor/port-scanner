use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};
use tokio::io::{AsyncReadExt, AsyncWriteExt};



const PAYLOAD_BANNER_GRABBING: &[u8] = b"GET / HTTP/1.0\r\n\r\n";
const TIMEOUT_BANNER_GRABBING: u16 = 100;


pub async fn probe(
    host: &str,
    port: u16,
    timeout_ms: u64,
) -> Option<String> {
   
    let addr = format!("{}:{}", host, port);
    let duration = Duration::from_millis(timeout_ms);

    match timeout(duration, TcpStream::connect(&addr)).await {
        Ok(Ok(mut stream)) => {
            let mut buffer = [0u8; 1024];

            if stream.write_all(PAYLOAD_BANNER_GRABBING).await.is_err() {
                return None;
            }

            let bytes_read = match timeout(Duration::from_millis(TIMEOUT_BANNER_GRABBING as u64), stream.read(&mut buffer)).await {
                Ok(Ok(n)) => n,
                _ => 0,
            };

            if bytes_read > 0 {
                let banner = String::from_utf8_lossy(&buffer[..bytes_read]);
                Some(banner.to_string())
            } else {
                None
            }
        }
        Ok(Err(_)) => {
            None
        }
        Err(_) => None,
    }
}