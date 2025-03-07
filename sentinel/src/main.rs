use sentinel::Sentinel;

mod config;
mod sentinel;
mod topic;
mod types;

#[tokio::main]
async fn main() {
    Sentinel::new().run().await;
}
