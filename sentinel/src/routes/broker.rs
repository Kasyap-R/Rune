use crate::metadata::Broker;
use crate::sentinel::Sentinel;
use common_lib::request_types::RegisterBrokerRequest;

use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

// TODO: Make handshake process more robust to ensure a broker can't register multiple times
#[post("/register_broker", format = "json", data = "<request>")]
pub async fn register_broker(
    request: Json<RegisterBrokerRequest>,
    sentinel: &State<Arc<Sentinel>>,
) -> Status {
    let mut metadata = sentinel.metadata.lock().await;
    let id = Uuid::new_v4();
    metadata.brokers.insert(
        id,
        Broker {
            addr: request.addr,
            last_heartbeat: Instant::now(),
        },
    );
    Status::Ok
}
