use std::sync::Arc;

use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio_serial::{
    available_ports, SerialPortBuilderExt, SerialPortInfo, SerialPortType, SerialStream,
};
use tracing::{error, info, warn};

use crate::common::app_error::AppError;
use crate::driver::adapter::Adapter;

const READY_MSG: &str = "ok"; // marlin firmware 'ready' message
const SERIAL_TIMEOUT: u64 = 5; // seconds

pub struct SerialAdapter {
    state: Arc<Mutex<SerialAdapterState>>,
}

struct SerialAdapterState {
    running: bool,
    printer: Option<String>,
    stream: Option<SerialStream>,
    status: Option<SerialAdapterStatus>,
}

enum SerialAdapterStatus {
    Ready,
    Printing,
    Paused,
    Error,
}

impl Default for SerialAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Adapter for SerialAdapter {
    async fn is_running(&self) -> bool {
        let state = self.state.lock().await;
        state.running
    }

    async fn configure(&mut self, printer: String) -> Result<(), AppError> {
        let mut state = self.state.lock().await;
        state.printer = Some(printer);
        Ok(())
    }

    async fn start(&mut self) -> Result<(), AppError> {
        let ports = available_ports().expect("No IO ports found!");
        let usb_port_count = SerialAdapter::get_usb_port_count(&ports).await;
        info!("Found {} USB ports", usb_port_count);
        if usb_port_count == 0 {
            warn!("No USB ports found, serial adapter requires active USB connections");
        } else {
            for port in ports {
                if let SerialPortType::UsbPort(usb_port_info) = port.port_type {
                    if port.port_name.contains("cu.") {
                        continue;
                    }
                    info!("Port: {}", port.port_name);
                    info!("USB device: {:?}", usb_port_info);
                }
            }
        }

        let printer = match self.state.lock().await.printer.clone() {
            Some(printer) => printer,
            None => {
                return Err(AppError::AdapterError {
                    message: "No printer configured".to_string(),
                });
            }
        };
        info!("Printer: {}", printer);
        SerialAdapter::handle_serial_stream(self, printer.to_string()).await?;
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    async fn send_command(&mut self, command: &str) -> Result<(), AppError> {
        let mut state = self.state.lock().await;
        let mut command = command.to_string();
        command.push('\n');

        match &mut state.stream {
            Some(stream) => {
                SerialAdapter::write_to_serial_stream(stream, command.as_bytes()).await?;
                info!("Command sent: {}", command.trim());
            }
            None => {
                return Err(AppError::AdapterError {
                    message: "No serial stream available".to_string(),
                });
            }
        };

        SerialAdapter::wait_until_printer_is_ready(state.stream.as_mut().unwrap()).await?;

        Ok(())
    }

    async fn start_print(&mut self, commands: Vec<String>) -> Result<(), AppError> {
        todo!()
    }

    async fn stop_print(&mut self) -> Result<(), AppError> {
        todo!()
    }

    async fn pause_print(&mut self) -> Result<(), AppError> {
        todo!()
    }

    async fn resume_print(&mut self) -> Result<(), AppError> {
        todo!()
    }
}

// Implementation of the SerialAdapter
impl SerialAdapter {
    pub fn new() -> Self {
        SerialAdapter {
            state: Arc::new(Mutex::new(SerialAdapterState {
                running: false,
                printer: None,
                status: None,
                stream: None,
            })),
        }
    }

    #[allow(clippy::unused_io_amount)] // we're controlling the buffer size ourselves
    async fn handle_serial_stream(&mut self, port_name: String) -> Result<(), AppError> {
        info!("Initializing serial communication on {}", &port_name);
        let stream = SerialAdapter::open_serial_stream(&port_name).await?;

        info!(
            "Initialized on {} with baud {} and buffer size {}",
            &port_name, 115_200, 256
        );

        self.state.lock().await.stream = Some(stream);
        self.state.lock().await.status = Some(SerialAdapterStatus::Ready);

        //set running to true
        self.state.lock().await.running = true;
        Ok(())
    }

    /// Open the serial port and return the SerialStream
    async fn open_serial_stream(port_name: &str) -> Result<SerialStream, AppError> {
        let port = tokio_serial::new(port_name, 115_200)
            .timeout(std::time::Duration::from_millis(250))
            .open_native_async();

        match port {
            Ok(port) => {
                info!("Serial port {} opened successfully", port_name);
                Ok(port)
            }
            Err(_) => Err(AppError::AdapterError {
                message: format!("Failed to open serial port: {}", port_name),
            }),
        }
    }

    /// Write to the SerialStream
    async fn write_to_serial_stream(port: &mut SerialStream, data: &[u8]) -> Result<(), AppError> {
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

    /// Read from the SerialStream and return the received data
    async fn read_serial_stream(port: &mut SerialStream) -> Result<String, AppError> {
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


    /// Wait until the printer is ready to receive commands or until the timeout is reached
    async fn wait_until_printer_is_ready(stream: &mut SerialStream) -> Result<(), AppError>
    {
        let start = std::time::Instant::now();

        loop {
            let data = SerialAdapter::read_serial_stream(stream).await?;
            if data.contains(READY_MSG) {
                info!("Printer is ready");
                break;
            }

            if start.elapsed().as_secs() >= SERIAL_TIMEOUT {
                error!("Printer serial stream timed out.");
                return Err(AppError::AdapterError {
                    message: "Printer serial stream timed out.".to_string(),
                });
            }
        }
        Ok(())
    }

    /// Helper function to count the number of USB ports in the list of available ports
    async fn get_usb_port_count(ports: &[SerialPortInfo]) -> usize {
        ports
            .iter()
            .filter(|port| matches!(port.port_type, SerialPortType::UsbPort(_)))
            .count()
    }
}


