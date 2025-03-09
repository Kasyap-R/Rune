use std::net::SocketAddr;

use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterBrokerRequest {
    pub addr: SocketAddr,
}
