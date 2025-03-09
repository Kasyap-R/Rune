use common_lib::types::BrokerID;

pub struct Topic {
    name: String,
    partition_count: u32,
    replicas: Vec<BrokerID>,
}
