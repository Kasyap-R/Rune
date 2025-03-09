use sentinel::Sentinel;
use std::sync::Arc;

#[macro_use]
extern crate rocket;

mod broker_endpoints;
mod config;
mod metadata;
mod routes;
mod sentinel;
mod topic;

#[tokio::main]
async fn main() {
    let sentinel = Sentinel::new();
    let rocket_server = build_rocket(&sentinel);
    run_services(sentinel, rocket_server).await;
}

// Builds and configures the Rocket server
fn build_rocket(sentinel: &Arc<Sentinel>) -> rocket::Rocket<rocket::Build> {
    let config = rocket::Config {
        port: sentinel.config.port, // Use the CLI-provided port
        address: "127.0.0.1".parse().unwrap(),
        ..rocket::Config::default()
    };

    rocket::custom(config)
        .mount("/", routes::get_routes()) // Mount all routes
        .manage(sentinel.clone()) // Share state (by cloning reference)
}

// Runs both Rocket and Sentinel concurrently
async fn run_services(sentinel: Arc<Sentinel>, rocket_server: rocket::Rocket<rocket::Build>) {
    tokio::select! {
        res = sentinel.run() => {
            match res {
                Ok(_) => println!("Sentinel core loop exited successfully."),
                Err(e) => println!("Sentinel core loop exited with error: {}", e)
            }
        }
        res = rocket_server.launch() => {
            println!("Rocket server exited: {:?}", res);
        }
    }
}
