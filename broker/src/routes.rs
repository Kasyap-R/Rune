use rocket::Route;

// We define all our routes in the routes module and use this function
// to retrieve them all
pub fn get_routes() -> Vec<Route> {
    routes![]
}
