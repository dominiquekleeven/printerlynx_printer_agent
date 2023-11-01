use serialport::{available_ports, SerialPortInfo, SerialPortType};
use tracing::{info, warn};

use crate::adapters::agent_adapter::AgentAdapter;
use crate::common::app_error::AppError;
pub struct SerialAgentAdapter {}

impl Default for SerialAgentAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SerialAgentAdapter {
    pub fn new() -> Self {
        SerialAgentAdapter {}
    }

    pub fn detect_printer() -> Self {
        todo!("Detect and create serial printer, by sending a test message to the port")
    }

    pub fn get_usb_port_count(ports: &[SerialPortInfo]) -> usize {
        ports
            .iter()
            .filter(|port| matches!(port.port_type, SerialPortType::UsbPort(_))).count()
    }
}

impl AgentAdapter for SerialAgentAdapter {
    fn setup(&self) -> Result<(), AppError> {
        info!("Setting up serial agent adapter");
        let ports = available_ports().expect("No io ports found!");
        let usb_port_count = SerialAgentAdapter::get_usb_port_count(&ports);

        info!("Found {} USB ports", usb_port_count);
        if usb_port_count == 0 {
            warn!("No USB ports found, serial adapter for the print agent requires active USB connections!");
            return Err(AppError::AdapterError {
                message: "No USB ports found, serial adapter for the print agent requires active USB connections!".to_string(),
            });
        }

        // on macOS, both the Callout (/dev/cu.*) and Dial-in ports (/dev/tty.*) are double-listed
        for port in ports {
            if let SerialPortType::UsbPort(usb_port_info) = port.port_type {
                if port.port_name.contains("cu.") {
                    continue;
                }
                info!("Port: {}", port.port_name);
                info!("USB device: {:?}", usb_port_info);
            }
        }

        Ok(())
    }

    fn teardown(&self) -> Result<(), AppError> {
        Ok(())
    }

    fn start(&self) -> Result<(), AppError> {
        Ok(())
    }
}
