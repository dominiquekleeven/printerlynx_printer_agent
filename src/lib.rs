use dotenvy::dotenv;
use serialport::{available_ports, SerialPortType};
use tracing::info;

pub mod common;
pub mod infra;

// const BAUD_RATE: u32 = 115_200;

pub async fn start() {
    dotenv().expect(".env file not found");
    tracing_subscriber::fmt().compact().with_target(true).init();
    info!("Starting up...");
    check_serial_connections();
}

/// Checks and logs the connected serial devices/ports.
pub fn check_serial_connections() {
    let ports = available_ports().expect("No ports found!");

    // on macOS, both the Callout (/dev/cu.*) and Dial-in ports (/dev/tty.*) are double-listed
    for port in ports {
        match port.port_type {
            SerialPortType::UsbPort(usb_port_info) => {

                // ignore callout ports, we're using the dial-in ports (similar to Arduino IDE)
                if port.port_name.contains("cu.") {
                    continue;
                }
                info!("Port: {}", port.port_name);
                info!("USB device: {:?}", usb_port_info);
            },
            _ => {}
        }

    }


}