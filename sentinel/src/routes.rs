mod broker;

use rocket::Route;

// We define all our routes in the routes module and use this function
// to retrieve them all
pub fn get_routes() -> Vec<Route> {
    routes![broker::register_broker]
}

fn get_broker_endpoint_url(broker_url: &str, broker_endpoint: &str) -> String {
    format!("{}/{}", broker_url, broker_endpoint)
}
