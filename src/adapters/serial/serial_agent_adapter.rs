use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::{available_ports, SerialPortBuilderExt, SerialPortInfo, SerialPortType, SerialStream};
use tracing::{info, warn};

use crate::adapters::agent_adapter::AgentAdapter;
use crate::common::app_error::AppError;
use crate::common::gcode_command::GcodeCommand::{AutoHome};

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

    pub async fn get_usb_port_count(ports: &[SerialPortInfo]) -> usize {
        ports
            .iter()
            .filter(|port| matches!(port.port_type, SerialPortType::UsbPort(_)))
            .count()
    }

    pub async fn open_serial_port(port_name: &str) -> Result<SerialStream, AppError>
    {
        let port = tokio_serial::new(port_name, 115_200)
            .timeout(std::time::Duration::from_millis(250))
            .open_native_async();

        match port {
            Ok(port) => {
                info!("Serial port {} opened successfully", port_name);
                Ok(port)
            }
            Err(e) => {
                warn!("Failed to open serial port: {}", e);
                Err(AppError::AdapterError {
                    message: format!("Failed to open serial port: {}", e),
                })
            }
        }
    }

    #[allow(clippy::unused_io_amount)] // we're controlling the buffer size ourselves
    pub async fn start_serial_comm(port_name: String) -> Result<(), AppError> {
        info!("Initializing serial communication on {}", &port_name);
        let mut port = SerialAgentAdapter::open_serial_port(&port_name).await?;
        let mut buf: Vec<u8> = vec![0; 256];

        info!(
            "Serial communication initialized on {} with baud {} and buffer size {}",
            &port_name, 115_200, 256
        );
        let mut received_data = String::new();
        let mut has_checked_comm = false;

        loop {
            if !has_checked_comm {
                port.write_all(AutoHome.value())
                    .await
                    .expect("Failed to write to serial port");
                info!("Sent auto home command to printer");
                has_checked_comm = true;
            }

            let n = port.read(&mut buf[..]).await.unwrap();
            if n == 0 {
                continue;
            }
            let s = String::from_utf8_lossy(&buf[..n]);
            received_data.push_str(&s); // Append the received data to the buffer

            if received_data.contains('\n') {
                info!("[Received]: {}", received_data.replace('\n', ""));

                // if 'ok'
                if received_data.contains("ok") {
                    info!("* Printer is ready for next command");
                }

                received_data.clear(); // Clear the buffer
            }
        }
    }

}

#[async_trait]
impl AgentAdapter for SerialAgentAdapter {
    async fn name(&self) -> String {
        "Serial IO Adapter".to_string()
    }

    async fn setup(&self) -> Result<(), AppError> {
        info!("Setting up serial agent adapter");
        let ports = available_ports().expect("No io ports found!");
        let usb_port_count = SerialAgentAdapter::get_usb_port_count(&ports).await;

        info!("Found {} USB ports", usb_port_count);
        if usb_port_count == 0 {
            warn!("No USB ports found, serial adapter for the print agent requires active USB connections!");
            return Err(AppError::AdapterError {
                message: "No USB ports found, serial adapter for the print agent requires active USB connections!".to_string(),
            });
        }

        for port in ports {
            if let SerialPortType::UsbPort(usb_port_info) = port.port_type {
                if port.port_name.contains("cu.") {
                    continue;
                }
                info!("Port: {}", port.port_name);
                info!("USB device: {:?}", usb_port_info);

                SerialAgentAdapter::start_serial_comm(port.port_name).await?;
            }
        }

        Ok(())
    }

    async fn teardown(&self) -> Result<(), AppError> {
        Ok(())
    }
}




