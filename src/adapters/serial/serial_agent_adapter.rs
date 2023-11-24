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
    fn name(&self) -> String {
        "Serial IO Adapter".to_string()
    }

    /// Start the serial agent adapter
    async fn start(&self) -> Result<(), AppError> {
        info!("Starting serial agent adapter");
        let ports = available_ports().expect("No io ports found!");
        let usb_port_count = SerialAgentAdapter::get_usb_port_count(&ports).await;

        info!("Found {} USB ports", usb_port_count);
        if usb_port_count == 0 {
            warn!("No USB ports found, serial adapter requires active USB connections");
            return Err(AppError::AdapterError {
                message: "No USB ports found, serial adapter requires active USB connections"
                    .to_string(),
            });
        }

        for port in ports {
            if let SerialPortType::UsbPort(usb_port_info) = port.port_type {
                if port.port_name.contains("cu.") {
                    continue;
                }
                info!("Port: {}", port.port_name);
                info!("USB device: {:?}", usb_port_info);

                SerialAgentAdapter::start_serial_communication(port.port_name).await?;
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
    async fn start_serial_communication(port_name: String) -> Result<(), AppError> {
        info!("Initializing serial communication on {}", &port_name);
        let mut open_port = SerialAgentAdapter::open_serial_port(&port_name).await?;

        info!(
            "Serial communication initialized on {} with baud {} and buffer size {}",
            &port_name, 115_200, 256
        );
        let mut has_checked_comm = false;

        loop {
            if !has_checked_comm {
                // We always send an auto home command to the printer to make sure it's in a known state.
                SerialAgentAdapter::write_serial_port(&mut open_port, AutoHome.value()).await?;

                // let test_gcode_map = get_gcode_map_from_file("test_files/cube.gcode").await?;
                // let total_commands = test_gcode_map.len();
                // info!("Test gcode written to memory, commands: {}", total_commands);
                // info!("Starting test print");
                //
                // let mut processed_commands_per_second = 0;
                // let mut time = std::time::Instant::now();
                //
                // for i in 0..total_commands {
                //     let command = test_gcode_map.get(&i).unwrap();
                //
                //     loop {
                //         let data = SerialAgentAdapter::read_serial_port(&mut open_port).await?;
                //         if data.contains("cold extrusion") {
                //             panic!("Cold extrusion error, aborting!")
                //         }
                //         if data.contains("ok") {
                //             break;
                //         }
                //     }
                //     SerialAgentAdapter::write_serial_port(&mut open_port, command.as_bytes())
                //         .await?;
                //     info!("Command sent: {}", command.trim());
                //     info!(
                //         "Progress {}/{}, {}%",
                //         i + 1,
                //         total_commands,
                //         ((i + 1) as f32 / total_commands as f32 * 100.0).round()
                //     );
                //
                //     processed_commands_per_second += 1;
                //     if processed_commands_per_second == 10 {
                //         let elapsed = time.elapsed().as_secs_f32();
                //         let commands_per_second = processed_commands_per_second as f32 / elapsed;
                //         info!("Commands/second: {}", commands_per_second);
                //         processed_commands_per_second = 0;
                //         time = std::time::Instant::now();
                //     }
                // }

                has_checked_comm = true;
            }
            SerialAgentAdapter::read_serial_port(&mut open_port).await?;
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
            Ok(_) => {}
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
