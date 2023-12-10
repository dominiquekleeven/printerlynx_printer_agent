use std::sync::Arc;

use dotenvy::dotenv;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::common::app_error::AppError;
use crate::common::gcode_command::GcodeCommand;
use crate::driver::adapter::Adapter;
use crate::driver::serial::serial_adapter::SerialAdapter;

pub mod common;
pub mod domain;
pub mod driver;
pub mod infra;

//  app state
pub struct AppState {
    adapters: Arc<Mutex<Vec<Box<dyn Adapter>>>>,
}



pub async fn start() {
    dotenv().expect("No environment variables set");
    tracing_subscriber::fmt().compact().with_target(true).init();
    info!("Starting up...");

    // currently hardcoded to use the serial adapter - will need to be configurable through CLI
    let adapter = SerialAdapter::default();
    start_adapter(Box::new(adapter))
        .await
        .expect("Failed to start adapter");
}

pub async fn connect_to_core() {
    info!("Connecting with backend");
}

pub async fn start_adapter(mut adapter: Box<dyn Adapter>) -> Result<(), AppError> {
    info!("Attempting to start specified adapter");
    let serial_port = std::env::var("PRINTER_SERIAL_PORT").expect("PRINTER_SERIAL_PORT not set");
    adapter.configure(serial_port).await?;

    loop {
        if !adapter.is_connected().await {
            info!("Adapter is not connected, attempting to connect");
            match adapter.start().await {
                Ok(_) => {
                    info!("Adapter is ready");
                    adapter.send_command(GcodeCommand::AutoHome.value()).await?;
                }
                Err(e) => {
                    warn!(
                        "Failed to start adapter, reason: {}, retrying in 10 seconds..",
                        e
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                    // wait 10 seconds before retrying
                }
            }
        }
    }
}
