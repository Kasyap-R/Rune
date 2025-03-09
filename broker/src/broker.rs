use crate::config::BrokerConfig;
use crate::sentinel_endpoints;
use common_lib::request_types::RegisterBrokerRequest;

use anyhow::{Context, Result};
use clap::Parser;
use reqwest::{self, Client};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

pub struct Broker {
    pub config: Arc<BrokerConfig>,
    pub client: Arc<Client>,
}

impl Broker {
    pub fn new() -> Arc<Self> {
        Arc::new(Broker {
            config: Arc::new(BrokerConfig::parse()),
            client: Arc::new(Client::new()),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let _ = self.handshake().await?;
        loop {
            break;
        }
        Ok(())
    }

    async fn handshake(&self) -> Result<()> {
        let endpoint = self.get_endpoint_url(sentinel_endpoints::REGISTER_BROKER);
        let request_data = RegisterBrokerRequest {
            addr: self.broker_addr(),
        };
        let _response = self
            .client
            .post(endpoint)
            .json(&request_data)
            .send()
            .await?
            .error_for_status()
            .context("Failed to register broker")?;
        Ok(())
    }

    fn get_endpoint_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.sentinel_url(), endpoint)
    }

    fn broker_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.config.port)
    }

    fn sentinel_url(&self) -> String {
        format!("http://{}", self.config.sentinel_addr)
    }
}
