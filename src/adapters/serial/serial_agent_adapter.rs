use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::{
    available_ports, SerialPortBuilderExt, SerialPortInfo, SerialPortType, SerialStream,
};
use tracing::{info, warn};

use crate::adapters::agent_adapter::AgentAdapter;
use crate::common::app_error::AppError;
use crate::common::gcode_command::GcodeCommand::AutoHome;

pub struct SerialAgentAdapter {}

impl Default for SerialAgentAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// AgentAdapter implementation for SerialAgentAdapter
#[async_trait]
impl AgentAdapter for SerialAgentAdapter {
    async fn name(&self) -> String {
        "Serial IO Adapter".to_string()
    }

    /// Setup the serial agent adapter
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

    /// Teardown the serial agent adapter
    async fn teardown(&self) -> Result<(), AppError> {
        Ok(())
    }
}

/// SerialAgentAdapter implementation
impl SerialAgentAdapter {
    pub fn new() -> Self {
        SerialAgentAdapter {}
    }

    /// Start the serial communication with the 3d printer on the given port
    /// Main loop for the serial communication
    #[allow(clippy::unused_io_amount)] // we're controlling the buffer size ourselves
    async fn start_serial_comm(port_name: String) -> Result<(), AppError> {
        info!("Initializing serial communication on {}", &port_name);
        let mut port = SerialAgentAdapter::open_serial_port(&port_name).await?;

        info!(
            "Serial communication initialized on {} with baud {} and buffer size {}",
            &port_name, 115_200, 256
        );
        let mut has_checked_comm = false;

        loop {
            if !has_checked_comm {
                // We always send an auto home command to the printer to make sure it's in a known state.
                SerialAgentAdapter::write_serial_port(&mut port, AutoHome.value()).await?;

                // let test_gcode_map = get_gcode_test_file().await?;
                // let total_commands = test_gcode_map.len();
                // info!("Test gcode written to memory, commands: {}", total_commands);
                // for i in 0..total_commands {
                //     let command = test_gcode_map.get(&i).unwrap();
                //     let mut data = String::new();
                //
                //     loop {
                //         data = SerialAgentAdapter::read_serial_port(&mut port).await?;
                //         if data.contains("cold extrusion") {
                //             panic!("Cold extrusion error, aborting!")
                //         }
                //         if data.contains("ok") {
                //             break;
                //         }
                //     }
                //     SerialAgentAdapter::write_serial_port(&mut port, command.as_bytes()).await?;
                //     info!("Progress {}/{}", i + 1, total_commands);
                // }

                has_checked_comm = true;
            }

            SerialAgentAdapter::read_serial_port(&mut port).await?;
        }
    }

    /// Open the serial port and return the SerialStream
    async fn open_serial_port(port_name: &str) -> Result<SerialStream, AppError> {
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

    async fn write_serial_port(port: &mut SerialStream, data: &[u8]) -> Result<(), AppError> {
        match port.write_all(data).await {
            Ok(_) => {
                let s = String::from_utf8_lossy(data).replace('\n', "");
                info!("[Sent]: {}", s);
            }
            Err(e) => {
                warn!("Failed to write to serial port: {}", e);
                return Err(AppError::AdapterError {
                    message: format!("Failed to write to serial port: {}", e),
                });
            }
        }
        Ok(())
    }

    /// Read from the serial port and return the received data
    async fn read_serial_port(port: &mut SerialStream) -> Result<String, AppError> {
        let mut buf: Vec<u8> = vec![0; 128];
        let mut received_data = String::new();
        let n = match port.read(&mut buf[..]).await {
            Ok(n) => n,
            Err(e) => {
                warn!("Failed to read from serial port: {}", e);
                return Err(AppError::AdapterError {
                    message: format!("Failed to read from serial port: {}", e),
                });
            }
        };

        if n == 0 {
            return Ok("\n".to_string()); // Return a newline if no data was received from the serial port (this is normal)
        }

        let s = String::from_utf8_lossy(&buf[..n]);
        received_data.push_str(&s); // Append the received data to the buffer

        if received_data.contains('\n') {
            info!("[Received]: {}", received_data.replace('\n', ""));
            return Ok(received_data);
        }
        Ok(received_data)
    }

    /// Get the number of USB ports from the given SerialPortInfo
    async fn get_usb_port_count(ports: &[SerialPortInfo]) -> usize {
        ports
            .iter()
            .filter(|port| matches!(port.port_type, SerialPortType::UsbPort(_)))
            .count()
    }
}
