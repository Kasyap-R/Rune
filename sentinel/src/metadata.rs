use crate::topic::Topic;
use common_lib::types::BrokerID;

use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    time::Instant,
};

#[derive(Default)]
pub struct ClusterMetadata {
    pub brokers: HashMap<BrokerID, Broker>,
    pub topics: HashSet<Topic>,
}

pub struct Broker {
    pub addr: SocketAddr,
    pub last_heartbeat: Instant,
}
