use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, about = "Fast TCP port scanner made in Rust")]
pub struct Args {
    #[arg(long, help = "Target IP or hostname")]
    pub host: String,

    #[arg(long, default_value_t = 200, help = "Timeout in ms between ports")]
    pub timeout: u64,

    #[arg(long,default_value_t = 2,help = "Number of concurrent tasks by thread")]
    pub threads: u16,

    #[arg(long, default_value = "1-1024", help = "Range of ports")]
    pub ports: String,
}

impl Args {
    pub fn map_ports(&self) -> Option<Vec<u16>> {
        let (start, end) = self.ports.split_once("-")?;

        let start: u16 = start.parse().ok()?;
        let end: u16 = end.parse().ok()?;

        Some((start..=end).collect())
    }
}