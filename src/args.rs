use clap::{ArgGroup, Parser};
use oura::utils::metrics;
use std::{
    default::Default,
    error::Error,
    fmt::{self, Display},
    ops::Deref,
    str::FromStr,
};

/// Cardano scripts dumper using Oura
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(ArgGroup::new("node")
    .required(true)
    .args(&["host", "socket"]),
))]
pub struct Args {
    /// Cardano node hostname or IP address
    #[clap(short, long, group = "node")]
    pub host: Option<String>,

    /// Enable Prometheus metrics{n} ('default' for 127.0.0.1:9188/metrics or ADDR:PORT/ENDPOINT)
    #[clap(short, long)]
    pub metrics: Option<Metrics>,

    /// Network ('mainnet', 'testnet' or magic)
    #[clap(short, long, default_value = "mainnet")]
    pub network: String,

    /// Output directory
    #[clap(short, long, default_value = "/tmp/scripts")]
    pub output: String,

    /// Cardano node port
    #[clap(short, long, default_value_t = 3001)]
    pub port: u16,

    /// Cardano node socket path
    #[clap(short, long, group = "node")]
    pub socket: Option<String>,

    /// Print scripts on standard output
    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub struct Metrics(metrics::Config);

#[derive(Debug)]
pub struct MetricsParseError;

impl Display for MetricsParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expecting ADDR:PORT/ENDPOINT (ex: 0.0.0.0:9188/metrics)")
    }
}

impl Error for MetricsParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl FromStr for Metrics {
    type Err = MetricsParseError;

    fn from_str(s: &str) -> Result<Metrics, MetricsParseError> {
        match s {
            "default" => Ok(Default::default()),
            _ => {
                let v: Vec<&str> = s.splitn(2, '/').collect();
                match v[..] {
                    [binding, endpoint] => Ok(Metrics(metrics::Config {
                        binding: Some(binding.to_owned()),
                        endpoint: Some(endpoint.to_owned()),
                    })),
                    _ => Err(MetricsParseError),
                }
            }
        }
    }
}

impl Default for Metrics {
    fn default() -> Metrics {
        Metrics(metrics::Config {
            binding: Some("127.0.0.1:9188".to_owned()),
            endpoint: Some("/metrics".to_owned()),
        })
    }
}

impl Deref for Metrics {
    type Target = metrics::Config;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
