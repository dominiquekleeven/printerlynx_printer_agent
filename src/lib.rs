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
    init_adapter(Box::new(adapter))
        .await
        .expect("Failed to initialize adapter");
}

pub async fn init_adapter(adapter: Box<dyn AgentAdapter>) -> Result<(), AppError> {
    info!("Initializing adapter: {}", adapter.name().await);

    //attempt to setup the adapter, if it fails, retry in 10 seconds

    loop {
        match adapter.setup().await {
            Ok(_) => {
                info!("Adapter setup finished");
                return Ok(());
            }
            Err(e) => {
                warn!("Failed to setup adapter: {}, retrying in 10 seconds..", e);
                tokio::time::sleep(std::time::Duration::from_secs(10)).await; // wait 10 seconds before retrying
            }
        }
    }
}
