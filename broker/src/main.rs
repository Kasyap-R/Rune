use std::sync::Arc;

use broker::Broker;

#[macro_use]
extern crate rocket;

mod broker;
mod config;
mod routes;
mod sentinel_endpoints;
mod topic;

#[tokio::main]
async fn main() {
    let broker = Broker::new();
    let rocket_server = build_rocket(&broker);
    run_services(broker, rocket_server).await;
}

// Builds and configures the Rocket server
fn build_rocket(broker: &Arc<Broker>) -> rocket::Rocket<rocket::Build> {
    let config = rocket::Config {
        port: broker.config.port, // Use the CLI-provided port
        address: "127.0.0.1".parse().unwrap(),
        ..rocket::Config::default()
    };

    rocket::custom(config)
        .mount("/", routes::get_routes()) // Mount all routes
        .manage(broker.clone()) // Share state (by cloning reference)
}

// Runs both Rocket and Broker concurrently
async fn run_services(broker: Arc<Broker>, rocket_server: rocket::Rocket<rocket::Build>) {
    tokio::select! {
        res = broker.run() => {
            match res {
                Ok(_) => println!("Broker core loop exited successfully."),
                Err(e) => println!("Broker core loop exited with error: {}", e)
            }
        }
        res = rocket_server.launch() => {
            println!("Rocket server exited: {:?}", res);
        }
    }
}
