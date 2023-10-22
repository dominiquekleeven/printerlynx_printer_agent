use dotenvy::dotenv;
use tracing::info;

pub mod common;

pub async fn start() {
    dotenv().expect(".env file not found");
    tracing_subscriber::fmt().compact().with_target(true).init();

    info!("Starting up...");

    todo!("Register the agent with the server, then start the agent.")
}