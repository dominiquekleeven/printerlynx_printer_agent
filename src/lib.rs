use crate::adapters::agent_adapter::AgentAdapter;
use crate::adapters::serial::serial_agent_adapter::SerialAgentAdapter;
use crate::common::app_error::AppError;
use dotenvy::dotenv;
use tracing::{info, warn};

pub mod adapters;
pub mod common;
pub mod domain;
pub mod infra;

pub async fn start() {
    dotenv().expect(".env file not found");
    tracing_subscriber::fmt().compact().with_target(true).init();
    info!("Starting up...");

    // currently hardcoded to use the serial adapter, use env vars to switch between adapters
    let adapter = SerialAgentAdapter::default();
    init_adapter(Box::new(adapter)).await.expect("Failed to initialize adapter");
}


pub async fn init_adapter(adapter: Box<dyn AgentAdapter>) -> Result<(), AppError> {
    match adapter.setup() {
        Ok(_) => {
            info!("Adapter setup finished");
            adapter.start().expect("Failed to start adapter");
            Ok(())
        }
        Err(e) => {
            warn!("Failed to setup adapter: {}", e);
            Err(e)
        }
    }
}
