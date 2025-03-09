use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
pub struct BrokerConfig {
    /// Port that the broker will be hosted on
    #[arg(short, long)]
    pub port: u16,

    /// Addr of the sentinel
    #[arg(short, long)]
    pub sentinel_addr: SocketAddr,
}
