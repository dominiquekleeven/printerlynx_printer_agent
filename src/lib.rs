use dotenvy::dotenv;
use tokio_serial::available_ports;
use tracing::info;

pub mod common;

pub async fn start() {
    dotenv().expect(".env file not found");
    tracing_subscriber::fmt().compact().with_target(true).init();

    info!("Starting up...");
    check_serial_connections();

    todo!("Register the agent with the server, then start the agent.")
}

pub fn check_serial_connections() -> bool {
    let ports = available_ports().expect("No ports found!");

    // It should also be noted that on macOS, both the Callout (/dev/cu.*) and Dial-in ports (/dev/tty.*)
    // ports are enumerated, resulting in two available ports per connected serial device.
    info!("Detected {} serial ports", ports.len());

    // list serial ports
    for p in ports {
        info!("Serial port: {:?}", p)
    }

    true
}