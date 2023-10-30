use dotenvy::dotenv;
use tokio_serial::{available_ports, SerialPortType};
use tracing::info;

pub mod common;
pub mod infra;

pub async fn start() {
    dotenv().expect(".env file not found");
    tracing_subscriber::fmt().compact().with_target(true).init();

    info!("Starting up...");
    check_serial_connections();
}

/// Checks and logs the connected serial devices/ports.
pub fn check_serial_connections() {
    let ports = available_ports().expect("No ports found!");

    // It should also be noted that on macOS, both the Callout (/dev/cu.*) and Dial-in ports (/dev/tty.*)
    // ports are enumerated, resulting in two available ports per connected serial device.
    info!("Detected {} serial ports", ports.len());

    // list serial ports
    for p in ports {
        match p.port_type {
            SerialPortType::UsbPort(usb_info) => {
                info!("Port name: {}", p.port_name);
                info!("USB device: {:?}", usb_info);
            }
            _ => {}
        }
    }
}