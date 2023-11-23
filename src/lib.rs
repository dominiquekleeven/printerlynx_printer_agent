use dotenvy::dotenv;
use tracing::{info, warn};

use crate::adapters::agent_adapter::AgentAdapter;
use crate::adapters::serial::serial_agent_adapter::SerialAgentAdapter;
use crate::common::app_error::AppError;

pub mod adapters;
pub mod common;
pub mod domain;
pub mod infra;

pub async fn start() {
    dotenv().expect(".env file not found");
    tracing_subscriber::fmt().compact().with_target(true).init();
    info!("Starting up...");

    // currently hardcoded to use the serial adapter, use env vars/toml/cli to switch between adapters
    let adapter = SerialAgentAdapter::new();
    start_adapter(Box::new(adapter))
        .await
        .expect("Failed to start adapter");
}

pub async fn start_adapter(adapter: Box<dyn AgentAdapter>) -> Result<(), AppError> {
    info!("Attempting to start adapter: {}", adapter.name());
    loop {
        match adapter.start().await {
            Ok(_) => {
                info!("Started adapter: {}", adapter.name());
                return Ok(());
            }
            Err(e) => {
                warn!(
                    "Failed to start adapter, reason: {}, retrying in 10 seconds..",
                    e
                );
                tokio::time::sleep(std::time::Duration::from_secs(10)).await; // wait 10 seconds before retrying
            }
        }
    }
}
