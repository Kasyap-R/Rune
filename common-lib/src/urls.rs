use std::net::SocketAddr;

pub fn broker_url(addr: &SocketAddr) -> String {
    format!("http://{}:{}", addr.ip(), addr.port())
}
