use crate::config::SentinelConfig;
use crate::topic::Topic;
use crate::types::BrokerID;
use clap::Parser;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    net::SocketAddr,
};

#[derive(Default)]
struct ClusterMetadata {
    brokers: HashMap<BrokerID, SocketAddr>,
    topics: HashSet<Topic>,
    free_ids: BinaryHeap<Reverse<BrokerID>>, // Stores freed up broker_ids (i.e. for when a broker
    // disconnects)
    next_broker_id: BrokerID,
}

pub struct Sentinel {
    config: SentinelConfig,
    metadata: ClusterMetadata,
}

impl Sentinel {
    pub fn new() -> Sentinel {
        Sentinel {
            config: SentinelConfig::parse(),
            metadata: ClusterMetadata::default(),
        }
    }

    pub async fn run(self) {
        println!(
            "Running Sentinel on port: {:#?}, storing metadata in: {:#?}",
            self.config.port, self.config.data_dir
        );
    }
}
