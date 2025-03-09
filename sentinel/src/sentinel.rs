use crate::config::SentinelConfig;
use crate::metadata::ClusterMetadata;

use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Client;
use std::sync::Arc;
use tokio::{
    sync::Mutex,
    time::{Duration, sleep},
};

pub struct Sentinel {
    pub config: Arc<SentinelConfig>,
    pub metadata: Arc<Mutex<ClusterMetadata>>,
    pub client: Arc<Client>,
}

impl Sentinel {
    pub fn new() -> Arc<Self> {
        Arc::new(Sentinel {
            config: Arc::new(SentinelConfig::parse()),
            metadata: Arc::new(Mutex::new(ClusterMetadata::default())),
            client: Arc::new(Client::new()),
        })
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            sleep(Duration::from_secs(1)).await;
        }
        Ok(())
    }
}
