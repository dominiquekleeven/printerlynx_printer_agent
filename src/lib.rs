use dotenvy::dotenv;
use serialport::{available_ports, SerialPortType};
use tracing::{info, warn};

pub mod adapters;
pub mod common;
pub mod domain;
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
    let usb_port_count = ports
        .iter()
        .filter(|port| match port.port_type {
            SerialPortType::UsbPort(_) => true,
            _ => false,
        })
        .count();

    info!("Found {} USB ports", usb_port_count);

    if usb_port_count == 0 {
        warn!("No USB ports found, serial adapter for the print agent requires active USB connections! Exiting...");
        return; // no USB ports found, no need to continue
    }

    // on macOS, both the Callout (/dev/cu.*) and Dial-in ports (/dev/tty.*) are double-listed
    for port in ports {
        if let SerialPortType::UsbPort(usb_port_info) = port.port_type {
            // ignore callout ports, we're using the dial-in ports (similar to Arduino IDE)
            if port.port_name.contains("cu.") {
                continue;
            }
            info!("Port: {}", port.port_name);
            info!("USB device: {:?}", usb_port_info);
        }
    }
}
