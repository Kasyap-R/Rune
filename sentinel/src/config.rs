use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct SentinelConfig {
    /// Port that sentinel will be hosted on
    #[arg(short, long)]
    pub port: u32,

    /// Path where cluster metadata will be stored
    #[arg(short, long)]
    pub data_dir: PathBuf,
}
